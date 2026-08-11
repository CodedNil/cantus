use crate::render::{
    shader::{pill_margin, pixel_to_ndc, quad_coord},
    shared::FrameData,
};
use isthmus::glam::{Vec2, Vec3, vec2};

#[cfg(target_arch = "spirv")]
use isthmus::spirv_std::num_traits::Float;

#[cfg(feature = "cpu")]
use {
    crate::render::cpu::Passes,
    arrayvec::ArrayVec,
    isthmus::Storage,
    ttf_parser::{Face, GlyphId, OutlineBuilder, Tag},
};

pub const MAX_LINE_GLYPHS: usize = 64;
pub const COLOR: Vec3 = Vec3::splat(0.94);
const EFFECT_PADDING: f32 = 3.5;

#[isthmus::data]
pub struct Edge {
    low_start: Vec2,
    low_control: Vec2,
    low_end: Vec2,
    high_start: Vec2,
    high_control: Vec2,
    high_end: Vec2,
}

#[isthmus::data]
#[derive(Default)]
pub struct Line {
    pub min: Vec2,
    pub max: Vec2,
    pub origin: Vec2,
    pub size: f32,
    pub weight: f32,
    pub count: u32,
    pub first: u32,
}

#[isthmus::data]
#[derive(Default)]
pub struct Glyph {
    min: Vec2,
    max: Vec2,
    start: u32,
    count: u32,
}

#[isthmus::data]
#[derive(Default)]
pub struct PlacedGlyph {
    pub x: f32,
    pub glyph: u32,
}

#[derive(isthmus::Varyings)]
pub struct Varyings {
    pub pixel: Vec2,
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

    #[cfg(feature = "cpu")]
    fn normalized_weight(self) -> f32 {
        ((self.weight - 600.0) / 300.0).clamp(0.0, 1.0)
    }
}

#[isthmus::outline]
fn quadratic(a: Vec2, control: Vec2, b: Vec2, t: f32) -> Vec2 {
    let u = 1.0 - t;
    a * u * u + control * (2.0 * u * t) + b * t * t
}

#[isthmus::outline]
fn ray_crossing(a: Vec2, control: Vec2, b: Vec2, point: Vec2, t: f32) -> i32 {
    if t < 0.0 || t >= 1.0 {
        return 0;
    }
    let curve = quadratic(a, control, b, t);
    if curve.x <= point.x {
        return 0;
    }
    let vertical_tangent = ((control.y - a.y) * (1.0 - t) + (b.y - control.y) * t) * 2.0;
    if vertical_tangent > 0.0 {
        1
    } else if vertical_tangent < 0.0 {
        -1
    } else {
        0
    }
}

/// Exact non-zero winding contribution of a quadratic to a rightward ray from `point`.
fn edge_winding(a: Vec2, control: Vec2, b: Vec2, point: Vec2) -> i32 {
    let quadratic = a.y - control.y * 2.0 + b.y;
    let linear = (control.y - a.y) * 2.0;
    let constant = a.y - point.y;
    if quadratic.abs() < 1e-7 {
        return if linear.abs() < 1e-7 {
            0
        } else {
            ray_crossing(a, control, b, point, -constant / linear)
        };
    }
    let discriminant = linear * linear - 4.0 * quadratic * constant;
    if discriminant <= 0.0 {
        return 0;
    }
    let root = discriminant.sqrt();
    let divisor = quadratic * 2.0;
    ray_crossing(a, control, b, point, (-linear - root) / divisor)
        + ray_crossing(a, control, b, point, (-linear + root) / divisor)
}

#[isthmus::outline]
#[allow(clippy::manual_clamp)]
fn edge_distance(edge: Edge, weight: f32, point: Vec2, best_distance: f32) -> (f32, i32) {
    let a = edge.low_start.lerp(edge.high_start, weight);
    let control = edge.low_control.lerp(edge.high_control, weight);
    let b = edge.low_end.lerp(edge.high_end, weight);
    let bounds_min = a.min(control).min(b);
    let bounds_max = a.max(control).max(b);
    let winding = if point.x >= bounds_max.x || point.y < bounds_min.y || point.y >= bounds_max.y {
        0
    } else {
        edge_winding(a, control, b, point)
    };
    if (point - point.clamp(bounds_min, bounds_max)).length_squared() >= best_distance {
        return (best_distance, winding);
    }
    let chord = b - a;
    let mut t = ((point - a).dot(chord) / chord.length_squared().max(1e-8))
        .max(0.0)
        .min(1.0);
    let second = (a - control * 2.0 + b) * 2.0;
    let mut iteration = 0;
    while iteration < 2 {
        let curve = quadratic(a, control, b, t);
        let tangent = ((control - a) * (1.0 - t) + (b - control) * t) * 2.0;
        let delta = curve - point;
        let denominator = tangent.length_squared() + delta.dot(second);
        let denominator = denominator.abs().max(1e-8).copysign(denominator);
        t = (t - delta.dot(tangent) / denominator).max(0.0).min(1.0);
        iteration += 1;
    }
    let distance = (point - a)
        .length_squared()
        .min((point - b).length_squared())
        .min((point - quadratic(a, control, b, t)).length_squared());
    (distance, winding)
}

fn glyph_distance(edges: &[Edge], start: u32, count: u32, weight: f32, point: Vec2, size: f32) -> f32 {
    let mut distance_squared = f32::MAX;
    let mut winding = 0;
    let mut index = 0;
    while index < count {
        let (distance, edge_winding) =
            edge_distance(edges[(start + index) as usize], weight, point, distance_squared);
        distance_squared = distance;
        winding += edge_winding;
        index += 1;
    }
    distance_squared.sqrt() * size * if winding == 0 { -1.0 } else { 1.0 }
}

#[allow(clippy::suspicious_operation_groupings)]
pub fn line_alpha(
    line: Line,
    placed_glyphs: &[PlacedGlyph],
    glyphs: &[Glyph],
    edges: &[Edge],
    local: Vec2,
) -> f32 {
    if (local.x < line.min.x || local.x > line.max.x) || (local.y < line.min.y || local.y > line.max.y) {
        return 0.0;
    }
    let mut low = 0;
    let mut high = line.count;
    while low < high {
        let middle = low + (high - low) / 2;
        let placed = placed_glyphs[(line.first + middle) as usize];
        if placed.x <= (local.x - line.origin.x) / line.size {
            low = middle + 1;
        } else {
            high = middle;
        }
    }
    let mut best: f32 = -1e6;
    // Include the next origin too: italic/curved glyphs and the effect padding can overhang left.
    let mut glyph_index = (low + 1).min(line.count);
    while glyph_index > 0 {
        glyph_index -= 1;
        let placed = placed_glyphs[(line.first + glyph_index) as usize];
        let glyph = glyphs[placed.glyph as usize];
        let point = vec2(
            (local.x - line.origin.x) / line.size - placed.x,
            -(local.y - line.origin.y) / line.size,
        );
        let padding = EFFECT_PADDING / line.size;
        if point.x > glyph.max.x + padding {
            break;
        }
        if point.x >= glyph.min.x - padding
            && point.y >= glyph.min.y - padding
            && point.x <= glyph.max.x + padding
            && point.y <= glyph.max.y + padding
        {
            best = best.max(glyph_distance(
                edges,
                glyph.start,
                glyph.count,
                line.weight,
                point,
                line.size,
            ));
        }
    }
    let coverage = (best * 1.25 + 0.5).clamp(0.0, 1.0);
    coverage * coverage * (3.0 - 2.0 * coverage)
}

pub fn vertex(line: Line, origin: Vec2, vertex: u32, frame: &FrameData) -> isthmus::Vertex<Varyings> {
    let margin = pill_margin(frame);
    let pixel = origin + line.min - margin + quad_coord(vertex) * (line.max - line.min + margin * 2.0);
    isthmus::Vertex {
        position: pixel_to_ndc(pixel, frame.screen_size),
        varyings: Varyings { pixel },
    }
}

#[cfg(feature = "cpu")]
const FONT: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../assets/NotoSans-Variable.ttf"
));
#[cfg(feature = "cpu")]
const WGHT: Tag = Tag::from_bytes(b"wght");
#[cfg(feature = "cpu")]
const RANGES: [(u32, u32); 8] = [
    (0x20, 0x7e),
    (0xa0, 0xff),
    (0x100, 0x17f),
    (0x300, 0x36f),
    (0x370, 0x3ff),
    (0x400, 0x4ff),
    (0x2000, 0x206f),
    (0x20ac, 0x20ac),
];

#[cfg(feature = "cpu")]
#[derive(Clone, Copy)]
struct Meta {
    glyph: u32,
    data: Glyph,
    advance: [f32; 2],
}

#[cfg(feature = "cpu")]
#[derive(Default)]
struct Outline {
    edges: Vec<[Vec2; 3]>,
    first: Vec2,
    current: Vec2,
}

#[cfg(feature = "cpu")]
impl Outline {
    fn segment(&mut self, point: Vec2) {
        self.edges
            .push([self.current, (self.current + point) * 0.5, point]);
        self.current = point;
    }
}

#[cfg(feature = "cpu")]
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

    #[allow(clippy::many_single_char_names)]
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let (start, a, b, end) = (self.current, vec2(x1, y1), vec2(x2, y2), vec2(x, y));
        for step in 1..=4 {
            let t = step as f32 / 4.0;
            let u = 1.0 - t;
            self.segment(
                start * u * u * u + a * 3.0 * u * u * t + b * 3.0 * u * t * t + end * t * t * t,
            );
        }
    }

    fn close(&mut self) {
        if self.current != self.first {
            self.segment(self.first);
        }
    }
}

#[cfg(feature = "cpu")]
pub struct ShapedLine {
    glyphs: ArrayVec<PlacedGlyph, MAX_LINE_GLYPHS>,
    min: Vec2,
    max: Vec2,
    pub(crate) width: f32,
    baseline: f32,
    size: f32,
    weight: f32,
}

#[cfg(feature = "cpu")]
pub struct Renderer {
    characters: Vec<(char, Meta)>,
    baseline: f32,
    edges: Storage<Edge>,
    glyphs: Storage<Glyph>,
    storage: Storage<PlacedGlyph>,
    placed: Vec<PlacedGlyph>,
}

#[cfg(feature = "cpu")]
impl Renderer {
    pub fn new(passes: &Passes<'_>, capacity: usize) -> Self {
        let mut face = Face::parse(FONT, 0).expect("parse variable font");
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
            face.set_variation(WGHT, weight)
                .expect("font must have a weight axis");
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
                        (
                            outline.edges,
                            f32::from(face.glyph_hor_advance(GlyphId(id)).unwrap_or(0)) / span,
                            (min, max),
                        )
                    })
                    .collect::<Vec<_>>(),
            );
        }
        let (mut curves, mut metadata) = (Vec::new(), Vec::new());
        for (index, id) in ids.iter().copied().enumerate() {
            let (low, low_advance, low_bounds) = &outlines[0][index];
            let (high, high_advance, high_bounds) = &outlines[1][index];
            assert_eq!(low.len(), high.len(), "variable outline topology changed");
            let count = low.len();
            let start = curves.len() as u32;
            for (&[a, b, c], &[d, e, f]) in low.iter().zip(high) {
                curves.push(Edge {
                    low_start: a,
                    low_control: b,
                    low_end: c,
                    high_start: d,
                    high_control: e,
                    high_end: f,
                });
            }
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
            .filter_map(|(character, id)| {
                metadata
                    .binary_search_by_key(&id, |&(id, _)| id)
                    .ok()
                    .map(|index| (character, metadata[index].1))
            })
            .collect();
        let edges = passes.storage("Vector Font Edges", curves);
        let glyphs = passes.storage("Vector Font Glyphs", metadata.iter().map(|(_, meta)| meta.data));
        Self {
            characters,
            baseline,
            edges,
            glyphs,
            storage: passes.storage_with_capacity("Text Glyphs", capacity),
            placed: Vec::with_capacity(capacity),
        }
    }

    fn glyph(&self, character: char) -> Option<Meta> {
        self.characters
            .binary_search_by_key(&character, |glyph| glyph.0)
            .ok()
            .map(|index| self.characters[index].1)
    }

    pub fn shape(&self, text: &str, style: TextStyle) -> ShapedLine {
        let mut min = Vec2::splat(f32::MAX);
        let mut max = Vec2::splat(f32::MIN);
        let weight = style.normalized_weight();
        let mut x = 0.0;
        let mut glyphs = ArrayVec::new();
        for meta in text.chars().filter_map(|character| self.glyph(character)) {
            if glyphs.len() < MAX_LINE_GLYPHS {
                min = min.min(vec2(x + meta.data.min.x, -meta.data.max.y));
                max = max.max(vec2(x + meta.data.max.x, -meta.data.min.y));
                glyphs.push(PlacedGlyph { x, glyph: meta.glyph });
            }
            x += meta.advance[0] + (meta.advance[1] - meta.advance[0]) * weight;
        }
        ShapedLine {
            glyphs,
            min,
            max,
            width: x * style.size,
            baseline: self.baseline * style.size,
            size: style.size,
            weight,
        }
    }
    pub const fn resources(&self) -> (&Storage<PlacedGlyph>, &Storage<Glyph>, &Storage<Edge>) {
        (&self.storage, &self.glyphs, &self.edges)
    }

    pub fn begin(&mut self) {
        self.placed.clear();
    }

    pub fn upload(&mut self) {
        self.storage.upload(&self.placed);
    }

    pub fn centered(&mut self, text: &str, style: TextStyle, center: Vec2) -> Line {
        let shaped = self.shape(text, style);
        let origin = vec2(center.x - shaped.width * 0.5, center.y + shaped.baseline);
        self.place(shaped, origin)
    }

    pub fn fit(&mut self, text: &str, style: TextStyle, y: f32, left: f32, right: f32) -> Line {
        let shaped = self.shape(text, style);
        self.fit_shaped(shaped, y, left, right)
    }

    pub fn fit_shaped(&mut self, shaped: ShapedLine, y: f32, left: f32, right: f32) -> Line {
        let x = if shaped.width <= right - left + 0.5 {
            (left + right - shaped.width) * 0.5
        } else {
            left
        };
        let baseline = shaped.baseline;
        self.place(shaped, vec2(x, y + baseline))
    }

    fn place(&mut self, shaped: ShapedLine, origin: Vec2) -> Line {
        let first = self.placed.len();
        let count = shaped.glyphs.len();
        let (min, max) = if count == 0 {
            (Vec2::ZERO, Vec2::ZERO)
        } else {
            (
                origin + shaped.min * shaped.size - EFFECT_PADDING,
                origin + shaped.max * shaped.size + EFFECT_PADDING,
            )
        };
        self.placed.extend(shaped.glyphs);
        Line {
            min,
            max,
            origin,
            size: shaped.size,
            weight: shaped.weight,
            count: count as u32,
            first: first as u32,
        }
    }
}
