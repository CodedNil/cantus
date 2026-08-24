#[cfg(not(target_arch = "spirv"))]
use crate::glam::Vec3;
use crate::{
    Fragment, Quad, Unorm8x4,
    glam::{Vec2, Vec4, vec2},
    rgba,
};

#[cfg(target_arch = "spirv")]
use crate::FloatExt;

#[cfg(not(target_arch = "spirv"))]
use {
    crate::backend::{buffer::BufferRange, canvas::Canvas, context::Context},
    smallvec::SmallVec,
    std::{ops::Range, sync::Arc, vec::Vec},
    ttf_parser::{Face, GlyphId, OutlineBuilder, Tag},
};

const EFFECT_PADDING: f32 = 3.5;
#[cfg(not(target_arch = "spirv"))]
const GLYPH_CAPACITY: usize = 16_384;

#[repr(C)]
#[derive(Clone, Copy, crate::ShaderData)]
struct F16x2 {
    value: u32,
}

impl F16x2 {
    #[cfg(not(target_arch = "spirv"))]
    fn new(value: Vec2) -> Self {
        Self {
            value: u32::from(half::f16::from_f32(value.x).to_bits()) | u32::from(half::f16::from_f32(value.y).to_bits()) << 16,
        }
    }

    fn get(self) -> Vec2 {
        #[cfg(target_arch = "spirv")]
        {
            spirv_std::float::f16x2_to_vec2(self.value)
        }
        #[cfg(not(target_arch = "spirv"))]
        {
            vec2(half::f16::from_bits(self.value as u16).to_f32(), half::f16::from_bits((self.value >> 16) as u16).to_f32())
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, crate::ShaderData)]
#[doc(hidden)]
pub struct Edge {
    start: F16x2,
    end: F16x2,
    start_delta: F16x2,
    end_delta: F16x2,
}

#[repr(C)]
#[derive(Clone, Copy, Default, crate::ShaderData)]
pub struct Line {
    pub min: Vec2,
    pub max: Vec2,
    pub origin: Vec2,
    pub size: f32,
    pub weight: f32,
    pub count: u32,
    pub first: u32,
    pub color: Unorm8x4,
}

impl Line {
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.count == 0
    }

    #[doc(hidden)]
    pub fn quad(self) -> Quad {
        Quad::from_min_max(self.min, self.max)
    }

    /// Extends rasterization beyond the font's normal effect padding.
    #[must_use]
    pub fn expanded(mut self, padding: f32) -> Self {
        self.min -= padding;
        self.max += padding;
        self
    }

    #[must_use]
    pub fn translated(mut self, offset: Vec2) -> Self {
        self.min += offset;
        self.max += offset;
        self.origin += offset;
        self
    }

    #[must_use]
    pub fn with_color(mut self, color: Vec4) -> Self {
        self.color = Unorm8x4::from_vec4(color);
        self
    }

    fn distance(self, placed_glyphs: &[PlacedGlyph], glyphs: &[Glyph], edges: &[Edge], point: Vec2) -> f32 {
        self.distance_scaled(placed_glyphs, glyphs, edges, point, 1.0)
    }

    fn distance_scaled(self, placed_glyphs: &[PlacedGlyph], glyphs: &[Glyph], edges: &[Edge], point: Vec2, scale: f32) -> f32 {
        line_distance_scaled(self, placed_glyphs, glyphs, edges, point, scale)
    }
}

/// Shader-side text geometry plus the fragment coordinates of its raster quad.
pub struct TextFragment<'a, Globals = ()> {
    pub pixel: Vec2,
    pub local: Vec2,
    pub uv: Vec2,
    pub time: f32,
    pub globals: Globals,
    pub line: Line,
    placed_glyphs: &'a [PlacedGlyph],
    glyphs: &'a [Glyph],
    edges: &'a [Edge],
}

impl<'a, Globals: Copy> TextFragment<'a, Globals> {
    #[doc(hidden)]
    pub const fn new(fragment: Fragment<Globals>, line: Line, placed_glyphs: &'a [PlacedGlyph], glyphs: &'a [Glyph], edges: &'a [Edge]) -> Self {
        Self {
            pixel: fragment.pixel,
            local: fragment.local,
            uv: fragment.uv,
            time: fragment.time,
            globals: fragment.globals,
            line,
            placed_glyphs,
            glyphs,
            edges,
        }
    }

    pub fn distance(&self, point: Vec2) -> f32 {
        self.line.distance(self.placed_glyphs, self.glyphs, self.edges, point)
    }

    pub fn distance_scaled(&self, point: Vec2, scale: f32) -> f32 {
        self.line.distance_scaled(self.placed_glyphs, self.glyphs, self.edges, point, scale)
    }

    pub fn distance_scaled_with_weight(&self, point: Vec2, scale: f32, weight: f32) -> f32 {
        let mut line = self.line;
        line.weight = weight;
        line.distance_scaled(self.placed_glyphs, self.glyphs, self.edges, point, scale)
    }

    pub fn alpha(&self) -> f32 {
        self.alpha_at(self.pixel)
    }

    pub fn alpha_at(&self, point: Vec2) -> f32 {
        coverage(self.distance(point))
    }

    pub fn color(&self, coverage: f32) -> Vec4 {
        let alpha = coverage * self.line.color.to_vec4().w;
        rgba(self.line.color.to_vec3(), alpha)
    }
}

#[repr(C)]
#[derive(Clone, Copy, crate::ShaderData)]
#[doc(hidden)]
pub struct Glyph {
    min: Vec2,
    max: Vec2,
    start: u32,
    count: u32,
}

#[repr(C)]
#[derive(Clone, Copy, crate::ShaderData)]
#[doc(hidden)]
pub struct PlacedGlyph {
    pub x: f32,
    pub glyph: u32,
}

#[derive(Clone, Copy)]
pub struct TextStyle {
    size: f32,
    weight: f32,
}

impl TextStyle {
    pub const fn new(size: f32, weight: f32) -> Self {
        Self { size, weight }
    }

    #[must_use]
    pub fn with_weight_mix(mut self, mix: f32) -> Self {
        self.weight = 600.0 + mix.clamp(0.0, 1.0) * 300.0;
        self
    }

    #[must_use]
    pub fn scaled(mut self, scale: f32) -> Self {
        self.size *= scale;
        self
    }

    #[cfg(not(target_arch = "spirv"))]
    fn normalized_weight(self) -> f32 {
        ((self.weight - 600.0) / 300.0).clamp(0.0, 1.0)
    }
}

fn edge_distance(edge: Edge, weight: f32, point: Vec2, best_distance: f32) -> (f32, i32) {
    let a = edge.start.get() + edge.start_delta.get() * weight;
    let b = edge.end.get() + edge.end_delta.get() * weight;
    let segment = b - a;
    let winding = if (a.y <= point.y && point.y < b.y) || (b.y <= point.y && point.y < a.y) {
        let crossing = a.x + (point.y - a.y) * segment.x / segment.y;
        if crossing > point.x { if segment.y > 0.0 { 1 } else { -1 } } else { 0 }
    } else {
        0
    };
    let bounds_min = a.min(b);
    let bounds_max = a.max(b);
    if (point - point.clamp(bounds_min, bounds_max)).length_squared() >= best_distance {
        return (best_distance, winding);
    }
    let t = ((point - a).dot(segment) / segment.length_squared().max(1e-8)).clamp(0.0, 1.0);
    ((point - (a + segment * t)).length_squared(), winding)
}

fn glyph_distance(edges: &[Edge], start: u32, count: u32, weight: f32, point: Vec2, size: f32) -> f32 {
    let mut distance_squared = f32::MAX;
    let mut winding = 0;
    // Rust-GPU cannot lower this runtime slice iterator without a pointer-to-integer conversion.
    for index in 0..count {
        let (distance, edge_winding) = edge_distance(edges[(start + index) as usize], weight, point, distance_squared);
        distance_squared = distance;
        winding += edge_winding;
    }
    let scaled_squared = distance_squared * size * size;
    let distance = if scaled_squared >= EFFECT_PADDING * EFFECT_PADDING {
        EFFECT_PADDING
    } else {
        scaled_squared.sqrt()
    };
    distance * if winding == 0 { -1.0 } else { 1.0 }
}

fn glyph_after(placed_glyphs: &[PlacedGlyph], first: u32, count: u32, x: f32) -> u32 {
    let mut low = 0;
    let mut high = count;
    while low < high {
        let middle = low + (high - low) / 2;
        if placed_glyphs[(first + middle) as usize].x <= x {
            low = middle + 1;
        } else {
            high = middle;
        }
    }
    low
}

fn line_distance_scaled(line: Line, placed_glyphs: &[PlacedGlyph], glyphs: &[Glyph], edges: &[Edge], local: Vec2, scale: f32) -> f32 {
    let inverse_size = 1.0 / line.size;
    let line_point = (local - line.origin) * inverse_size;
    let after = glyph_after(placed_glyphs, line.first, line.count, line_point.x);
    let mut best: f32 = -1e6;
    let padding = EFFECT_PADDING * inverse_size / scale;
    // Include the next origin too: italic/curved glyphs and the effect padding can overhang left.
    let mut glyph_index = (after + 1).min(line.count);
    while glyph_index > 0 {
        glyph_index -= 1;
        let placed = placed_glyphs[(line.first + glyph_index) as usize];
        let glyph = glyphs[placed.glyph as usize];
        let glyph_point = vec2(line_point.x - placed.x, -line_point.y) / scale;
        if glyph_point.x > glyph.max.x + padding {
            break;
        }
        if glyph_point.x >= glyph.min.x - padding && glyph_point.y >= glyph.min.y - padding && glyph_point.x <= glyph.max.x + padding && glyph_point.y <= glyph.max.y + padding {
            best = best.max(glyph_distance(edges, glyph.start, glyph.count, line.weight, glyph_point, line.size * scale));
        }
    }
    best
}

pub fn coverage(distance: f32) -> f32 {
    let coverage = (distance * 1.25 + 0.5).clamp(0.0, 1.0);
    coverage * coverage * (3.0 - 2.0 * coverage)
}

#[cfg(not(target_arch = "spirv"))]
const WGHT: Tag = Tag::from_bytes(b"wght");
#[cfg(not(target_arch = "spirv"))]
const RANGES: &[(u32, u32)] = &[
    (0x20, 0x7e),
    (0xa0, 0xff),
    (0x100, 0x17f),
    (0x300, 0x36f),
    (0x370, 0x3ff),
    (0x400, 0x4ff),
    (0x2000, 0x206f),
    (0x20ac, 0x20ac),
    (0x266a, 0x266b),
];

#[cfg(not(target_arch = "spirv"))]
#[derive(Clone, Copy)]
struct Meta {
    glyph: u32,
    data: Glyph,
    advance: [f32; 2],
}

#[cfg(not(target_arch = "spirv"))]
#[derive(Default)]
struct Outline {
    edges: Vec<[Vec2; 3]>,
    first: Vec2,
    current: Vec2,
}

#[cfg(not(target_arch = "spirv"))]
impl Outline {
    fn segment(&mut self, point: Vec2) {
        self.edges.push([self.current, self.current.midpoint(point), point]);
        self.current = point;
    }
}

#[cfg(not(target_arch = "spirv"))]
impl OutlineBuilder for Outline {
    fn move_to(&mut self, x: f32, y: f32) {
        self.first = vec2(x, y);
        self.current = self.first;
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.segment(vec2(x, y));
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let end = vec2(x, y);
        self.edges.push([self.current, vec2(x1, y1), end]);
        self.current = end;
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let (start, control_a, control_b, end) = (self.current, vec2(x1, y1), vec2(x2, y2), vec2(x, y));
        for step in 1..=4 {
            let phase = step as f32 / 4.0;
            let inverse = 1.0 - phase;
            self.segment(
                start * inverse * inverse * inverse + control_a * 3.0 * inverse * inverse * phase + control_b * 3.0 * inverse * phase * phase + end * phase * phase * phase,
            );
        }
    }

    fn close(&mut self) {
        if self.current != self.first {
            self.segment(self.first);
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
#[derive(Default)]
pub struct ShapedLine {
    glyphs: SmallVec<[PlacedGlyph; 32]>,
    min: Vec2,
    max: Vec2,
    pub width: f32,
    baseline: f32,
    size: f32,
    weight: f32,
}

#[cfg(not(target_arch = "spirv"))]
#[derive(Clone)]
pub struct Shaper {
    characters: Arc<[(char, Meta)]>,
    baseline: f32,
}

#[cfg(not(target_arch = "spirv"))]
pub(crate) struct Text {
    shaper: Shaper,
    context: Context,
    placed: Vec<PlacedGlyph>,
    color: Unorm8x4,
}

#[cfg(not(target_arch = "spirv"))]
pub struct TextLayout<'a> {
    text: &'a mut Text,
    shaped: ShapedLine,
}

#[cfg(not(target_arch = "spirv"))]
pub struct TextScope<'a> {
    text: &'a mut Text,
}

#[cfg(not(target_arch = "spirv"))]
impl<'a> TextScope<'a> {
    pub(crate) const fn new(text: &'a mut Text) -> Self {
        Self { text }
    }

    pub fn line(&mut self, content: &str, style: TextStyle) -> TextLayout<'_> {
        TextLayout::new(self.text, content, style)
    }

    pub fn width(&self, content: &str, style: TextStyle) -> f32 {
        self.text.width(content, style)
    }

    pub fn fit(&mut self, shaped: &ShapedLine, y: f32, bounds: Range<f32>) -> Line {
        self.text.fit_shaped(shaped, y, bounds.start, bounds.end)
    }

    pub fn visible(&mut self, shaped: &ShapedLine, left: Vec2, clip: Range<f32>) -> Line {
        self.text.place_visible(shaped, left, clip)
    }

    pub fn shaper(&self) -> Shaper {
        self.text.shaper()
    }
}

#[cfg(not(target_arch = "spirv"))]
impl<'a> TextLayout<'a> {
    pub(crate) fn new(text: &'a mut Text, content: &str, style: TextStyle) -> Self {
        let shaped = text.shaper.shape(content, style);
        Self { text, shaped }
    }

    pub fn shape(self) -> ShapedLine {
        self.shaped
    }

    pub fn centered(self, center: Vec2) -> Line {
        let origin = vec2(center.x - self.shaped.width * 0.5, center.y + self.shaped.baseline);
        self.text.place(&self.shaped, origin)
    }

    pub fn left(self, position: Vec2) -> Line {
        let baseline = self.shaped.baseline;
        self.text.place(&self.shaped, vec2(position.x, position.y + baseline))
    }

    pub fn right(self, position: Vec2) -> Line {
        let baseline = self.shaped.baseline;
        self.text.place(&self.shaped, vec2(position.x - self.shaped.width, position.y + baseline))
    }

    pub fn fit(self, y: f32, bounds: Range<f32>) -> Line {
        self.text.fit_shaped(&self.shaped, y, bounds.start, bounds.end)
    }

    pub fn visible(self, left: Vec2, clip: Range<f32>) -> Line {
        self.text.place_visible(&self.shaped, left, clip)
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Text {
    /// Creates the shared vector-font renderer and GPU storage.
    ///
    /// # Panics
    ///
    /// Panics if the supplied variable font cannot be parsed or its outlines cannot be read.
    pub(crate) fn new(canvas: &mut Canvas, font: &[u8], color: Vec3) -> Self {
        let mut face = Face::parse(font, 0).expect("parse variable font");
        let characters = RANGES
            .iter()
            .flat_map(|&(a, b)| a..=b)
            .filter_map(char::from_u32)
            .filter_map(|character| face.glyph_index(character).map(|id| (character, id.0)))
            .collect::<Vec<_>>();
        let mut ids = characters.iter().map(|&(_, id)| id).collect::<Vec<_>>();
        ids.sort_unstable();
        ids.dedup();
        let span = f32::from(face.ascender() - face.descender());
        let baseline = f32::from(face.ascender() + face.descender()) * 0.5 / span;
        let mut outlines = Vec::new();
        for weight in [600.0, 900.0] {
            face.set_variation(WGHT, weight).expect("font must have a weight axis");
            outlines.push(
                ids.iter()
                    .map(|&id| {
                        let mut outline = Outline::default();
                        let bounds = face.outline_glyph(GlyphId(id), &mut outline);
                        for edge in &mut outline.edges {
                            *edge = edge.map(|point| point / span);
                        }
                        let (min, max) = bounds.map_or((Vec2::ZERO, Vec2::ZERO), |bounds| {
                            (
                                vec2(f32::from(bounds.x_min), f32::from(bounds.y_min)) / span,
                                vec2(f32::from(bounds.x_max), f32::from(bounds.y_max)) / span,
                            )
                        });
                        (outline.edges, f32::from(face.glyph_hor_advance(GlyphId(id)).unwrap_or(0)) / span, (min, max))
                    })
                    .collect::<Vec<_>>(),
            );
        }
        let (mut curves, mut metadata) = (Vec::new(), Vec::new());
        for (index, id) in ids.iter().copied().enumerate() {
            let (low, low_advance, low_bounds) = &outlines[0][index];
            let (high, high_advance, high_bounds) = &outlines[1][index];
            assert_eq!(low.len(), high.len(), "variable outline topology changed");
            let start = curves.len() as u32;
            for (&[a, b, c], &[d, e, f]) in low.iter().zip(high) {
                // Flatten once on the CPU. The tolerance is in ems and keeps the
                // largest text comfortably below a quarter pixel of curve error.
                let curvature = (a - b * 2.0 + c).length().max((d - e * 2.0 + f).length());
                let segments = (curvature / 0.02).sqrt().ceil().max(1.0) as u32;
                let point = |start: Vec2, control: Vec2, end: Vec2, t: f32| {
                    let one_minus_t = 1.0 - t;
                    start * one_minus_t * one_minus_t + control * 2.0 * one_minus_t * t + end * t * t
                };
                for segment in 0..segments {
                    let t0 = segment as f32 / segments as f32;
                    let t1 = (segment + 1) as f32 / segments as f32;
                    let low_start = point(a, b, c, t0);
                    let low_end = point(a, b, c, t1);
                    let high_start = point(d, e, f, t0);
                    let high_end = point(d, e, f, t1);
                    curves.push(Edge {
                        start: F16x2::new(low_start),
                        end: F16x2::new(low_end),
                        start_delta: F16x2::new(high_start - low_start),
                        end_delta: F16x2::new(high_end - low_end),
                    });
                }
            }
            let count = curves.len() - start as usize;
            metadata.push((
                id,
                Meta {
                    glyph: index as u32,
                    data: Glyph {
                        start,
                        count: count as u32,
                        min: low_bounds.0.min(high_bounds.0),
                        max: low_bounds.1.max(high_bounds.1),
                    },
                    advance: [*low_advance, *high_advance],
                },
            ));
        }
        let characters = characters
            .into_iter()
            .filter_map(|(character, id)| metadata.binary_search_by_key(&id, |&(id, _)| id).ok().map(|index| (character, metadata[index].1)))
            .collect::<Vec<_>>();
        let context = canvas.context();
        let edges = context.upload_static(&curves);
        let glyphs = context.upload_static(&metadata.iter().map(|(_, meta)| meta.data).collect::<Vec<_>>());
        canvas.register_text(glyphs, edges);
        Self {
            shaper: Shaper {
                characters: characters.into(),
                baseline,
            },
            context,
            placed: Vec::with_capacity(GLYPH_CAPACITY),
            color: Unorm8x4::from_vec3(color),
        }
    }

    pub(crate) fn shaper(&self) -> Shaper {
        self.shaper.clone()
    }

    pub(crate) fn width(&self, content: &str, style: TextStyle) -> f32 {
        self.shaper.width(content, style)
    }

    pub(crate) fn begin_frame(&mut self) {
        self.placed.clear();
    }

    pub(crate) fn finish_frame(&self) -> BufferRange {
        self.context.upload(&self.placed)
    }

    pub(crate) fn place_visible(&mut self, shaped: &ShapedLine, left: Vec2, clip: Range<f32>) -> Line {
        let origin = vec2(left.x, left.y + shaped.baseline);
        let local = |x| (x - origin.x) / shaped.size;
        let start = shaped.glyphs.partition_point(|glyph| glyph.x < local(clip.start - EFFECT_PADDING)).saturating_sub(1);
        let end = (shaped.glyphs.partition_point(|glyph| glyph.x <= local(clip.end + EFFECT_PADDING)) + 1).min(shaped.glyphs.len());
        let mut line = self.place_range(shaped, origin, start..end);
        line.min.x = line.min.x.max(clip.start);
        line.max.x = line.max.x.min(clip.end);
        line
    }

    pub(crate) fn fit_shaped(&mut self, shaped: &ShapedLine, y: f32, left: f32, right: f32) -> Line {
        let x = if shaped.width <= right - left + 0.5 {
            (left + right - shaped.width) * 0.5
        } else {
            left
        };
        self.place(shaped, vec2(x, y + shaped.baseline))
    }

    fn place(&mut self, shaped: &ShapedLine, origin: Vec2) -> Line {
        self.place_range(shaped, origin, 0..shaped.glyphs.len())
    }

    fn place_range(&mut self, shaped: &ShapedLine, origin: Vec2, range: Range<usize>) -> Line {
        let first = self.placed.len();
        let count = range.len();
        let (min, max) = if count == 0 {
            (Vec2::ZERO, Vec2::ZERO)
        } else {
            (origin + shaped.min * shaped.size - EFFECT_PADDING, origin + shaped.max * shaped.size + EFFECT_PADDING)
        };
        self.placed.extend_from_slice(&shaped.glyphs[range]);
        Line {
            min,
            max,
            origin,
            size: shaped.size,
            weight: shaped.weight,
            count: count as u32,
            first: first as u32,
            color: self.color,
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl Shaper {
    fn glyph(&self, character: char) -> Option<Meta> {
        self.characters.binary_search_by_key(&character, |glyph| glyph.0).ok().map(|index| self.characters[index].1)
    }

    pub fn shape(&self, text: &str, style: TextStyle) -> ShapedLine {
        self.shape_positioned([(text, 0.0)], style, usize::MAX)
    }

    pub fn width(&self, text: &str, style: TextStyle) -> f32 {
        let weight = style.normalized_weight();
        text.chars()
            .filter_map(|character| self.glyph(character))
            .map(|meta| meta.advance[0] + (meta.advance[1] - meta.advance[0]) * weight)
            .sum::<f32>()
            * style.size
    }

    pub fn shape_positioned<'a>(&self, parts: impl IntoIterator<Item = (&'a str, f32)>, style: TextStyle, max_glyphs: usize) -> ShapedLine {
        let mut min = Vec2::splat(f32::MAX);
        let mut max = Vec2::splat(f32::MIN);
        let weight = style.normalized_weight();
        let mut width: f32 = 0.0;
        let mut glyphs = SmallVec::new();
        for (text, position) in parts {
            let mut x = position / style.size;
            for meta in text.chars().filter_map(|character| self.glyph(character)) {
                if glyphs.len() == max_glyphs {
                    break;
                }
                if meta.data.count > 0 {
                    min = min.min(vec2(x + meta.data.min.x, -meta.data.max.y));
                    max = max.max(vec2(x + meta.data.max.x, -meta.data.min.y));
                    glyphs.push(PlacedGlyph { x, glyph: meta.glyph });
                }
                x += meta.advance[0] + (meta.advance[1] - meta.advance[0]) * weight;
            }
            width = width.max(x * style.size);
        }
        ShapedLine {
            glyphs,
            min,
            max,
            width,
            baseline: self.baseline * style.size,
            size: style.size,
            weight,
        }
    }
}
