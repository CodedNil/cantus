use crate::render::{
    FrameData, GAP, PADDING,
    shader::{
        fill, pill_fragment, pill_vertex, pixel_to_ndc, presence, quad_coord, sd_rounded_box,
        segment_distance, stroke,
    },
    text,
};
use isthmus::{
    Sampler, Texture2DArray, Vertex,
    glam::{Vec2, Vec3, Vec4, vec2, vec3},
    spirv_std::arch::kill,
};

#[cfg(feature = "cpu")]
use {
    crate::{
        app::{
            Background,
            interaction::Rect,
            platform::{Current as Platform, DesktopApp, Platform as _},
        },
        render::{
            cpu::{Frame, Passes},
            smoothstep,
            text::TextStyle,
        },
    },
    fend_core::Context,
    image::imageops::FilterType,
    isthmus::{FilterableFloatFormat, SampledTexture, wgpu::Extent3d},
    resvg::{
        render,
        tiny_skia::{Pixmap, Transform},
        usvg::{self, Tree},
    },
    std::{collections::HashMap, error::Error, fs, ops::Range, path::Path, sync::OnceLock},
    tracing::warn,
    ureq::Agent,
};

const PANEL_WIDTH: f32 = 520.0;
/// Matched-app/calculator rows shown below the search bar.
pub(crate) const MAX_VISIBLE: usize = 8;

/// `LauncherRow::icon` sentinels; any other value indexes the icon texture array. `PANEL` marks
/// the backdrop, which also draws the search field on itself.
const PANEL: i32 = -3;
const CALCULATOR_ICON: i32 = -2;

/// Side of the square icon tile at the left of every row.
const ICON_SIZE: f32 = 32.0;
const BADGE_HEIGHT: f32 = 21.0;
/// Icons, badge outlines and the magnifier all share one grey.
const ICON_COLOR: Vec3 = Vec3::splat(0.58);
const ACCENT_COLOR: Vec3 = vec3(0.44, 0.40, 0.80);

#[cfg(feature = "cpu")]
mod host {
    use super::{TextStyle, Vec4};

    pub const BADGE_WIDTHS: [f32; 2] = [27.0, 42.0];
    pub const ICON_PX: u32 = 48;
    pub const MAX_ICON_SLOTS: usize = 192;
    pub const SEARCH_STYLE: TextStyle = TextStyle::new(18.0, 600.0);
    pub const NAME_STYLE: TextStyle = TextStyle::new(16.0, 700.0);
    pub const DETAIL_STYLE: TextStyle = TextStyle::new(13.0, 600.0);
    pub const DETAIL_COLOR: Vec4 = Vec4::new(0.56, 0.63, 0.86, 1.0);
    pub const MUTED_COLOR: Vec4 = Vec4::new(0.52, 0.55, 0.64, 1.0);
}

#[cfg(feature = "cpu")]
use host::{
    BADGE_WIDTHS, DETAIL_COLOR, DETAIL_STYLE, ICON_PX, MAX_ICON_SLOTS, MUTED_COLOR, NAME_STYLE,
    SEARCH_STYLE,
};

/// Height of the search field, which sits flush at the top of the panel instead of in a pill.
fn header_height(frame: &FrameData) -> f32 {
    frame.panel_height + PADDING
}

/// Origin and size of the flat background panel, centered in the surface.
fn background_bounds(frame: &FrameData) -> (Vec2, Vec2) {
    let rows = MAX_VISIBLE as f32;
    let height = header_height(frame) + PADDING * 2.0 + rows * frame.panel_height + (rows - 1.0) * GAP;
    let size = vec2(PANEL_WIDTH, height);
    ((frame.screen_size - size) * 0.5, size)
}

/// Left edge and width shared by every row; only `y` varies between them.
fn row_bounds(frame: &FrameData) -> (f32, f32) {
    let (origin, size) = background_bounds(frame);
    (origin.x + PADDING, size.x - PADDING * 2.0)
}

/// Coverage of a magnifying glass centered on the origin.
fn magnifier_icon(point: Vec2) -> f32 {
    let ring = stroke(point.length() - 6.2, 1.05);
    let handle = stroke(segment_distance(point, vec2(4.6, 4.6), vec2(8.8, 8.8)), 1.05);
    ring.max(handle)
}

/// Straight color and coverage of the calculator badge shown beside a fend answer.
fn calculator_icon(point: Vec2) -> Vec4 {
    let badge = fill(sd_rounded_box(point, Vec2::splat(13.0), 9.0));
    let bar = |offset: f32| fill(sd_rounded_box(point - vec2(0.0, offset), vec2(5.4, 1.1), 1.1));
    let equals = bar(-3.1).max(bar(3.1));
    ACCENT_COLOR
        .lerp(Vec3::splat(0.96), equals)
        .extend(badge.max(equals * badge))
}

/// "↵" or "⇧" glyph coverage, drawn around the origin.
fn key_glyph(point: Vec2, shift: bool) -> f32 {
    let distance = if shift {
        segment_distance(point, vec2(0.0, -4.0), vec2(-3.4, 0.2))
            .min(segment_distance(point, vec2(0.0, -4.0), vec2(3.4, 0.2)))
            .min(segment_distance(point, vec2(0.0, -0.6), vec2(0.0, 4.0)))
    } else {
        segment_distance(point, vec2(3.4, -3.6), vec2(3.4, 1.8))
            .min(segment_distance(point, vec2(3.4, 1.8), vec2(-2.6, 1.8)))
            .min(segment_distance(point, vec2(-2.6, 1.8), vec2(0.2, -0.8)))
            .min(segment_distance(point, vec2(-2.6, 1.8), vec2(0.2, 4.4)))
    };
    stroke(distance, 0.8)
}

/// Straight color and coverage of one key badge; `half_width` of 0 leaves the slot empty.
fn action_badge(point: Vec2, half_width: f32, shift: bool) -> Vec4 {
    if half_width <= 0.0 {
        return Vec4::ZERO;
    }
    let outline = sd_rounded_box(point, vec2(half_width, BADGE_HEIGHT * 0.5), 6.0);
    let (body, edge) = (fill(outline), stroke(outline, 0.65));
    let glyph = if shift {
        key_glyph(point + vec2(8.5, 0.0), true).max(key_glyph(point - vec2(7.5, 0.0), false))
    } else {
        key_glyph(point, false)
    };
    let color = Vec3::splat(0.27).lerp(ICON_COLOR, edge).lerp(text::COLOR, glyph);
    color.extend(body.max(edge).max(glyph))
}

/// The search query with a caret and selection, edited like an ordinary text box.
#[cfg(feature = "cpu")]
#[derive(Default)]
pub struct TextField {
    pub text: String,
    /// Byte offset of the caret, and of the other end of the selection.
    cursor: usize,
    anchor: usize,
    /// Frame time the caret blink last restarted at, so typing keeps it solid.
    blink_start: f32,
    /// Set by an edit, consumed by the next frame to restart the blink.
    touched: bool,
}

#[cfg(feature = "cpu")]
impl TextField {
    pub fn clear(&mut self) {
        self.text.clear();
        self.set_cursor(0, false);
    }

    pub const fn selection(&self) -> Range<usize> {
        if self.cursor < self.anchor {
            self.cursor..self.anchor
        } else {
            self.anchor..self.cursor
        }
    }

    pub fn selected_text(&self) -> &str {
        &self.text[self.selection()]
    }

    pub const fn set_cursor(&mut self, index: usize, select: bool) {
        self.cursor = index;
        if !select {
            self.anchor = index;
        }
        self.touched = true;
    }

    pub const fn select_all(&mut self) {
        self.anchor = 0;
        self.set_cursor(self.text.len(), true);
    }

    /// Where the caret lands moving one character in `forward`'s direction.
    fn step(&self, forward: bool) -> usize {
        let (before, after) = self.text.split_at(self.cursor);
        if forward {
            self.cursor + after.chars().next().map_or(0, char::len_utf8)
        } else {
            self.cursor - before.chars().next_back().map_or(0, char::len_utf8)
        }
    }

    /// Removes the selected text, reporting whether there was any.
    pub fn delete_selection(&mut self) -> bool {
        let range = self.selection();
        if range.is_empty() {
            return false;
        }
        self.text.replace_range(range.clone(), "");
        self.set_cursor(range.start, false);
        true
    }

    pub fn insert(&mut self, insertion: &str) {
        self.delete_selection();
        self.text.insert_str(self.cursor, insertion);
        self.set_cursor(self.cursor + insertion.len(), false);
    }

    /// Deletes the selection, or one character in `forward`'s direction.
    pub fn erase(&mut self, forward: bool) {
        if self.delete_selection() {
            return;
        }
        let target = self.step(forward);
        let (start, end) = (target.min(self.cursor), target.max(self.cursor));
        self.text.replace_range(start..end, "");
        self.set_cursor(start, false);
    }

    /// Moves the caret, collapsing an existing selection unless `select` extends it.
    pub fn move_cursor(&mut self, forward: bool, select: bool) {
        let range = self.selection();
        let target = match () {
            () if select || range.is_empty() => self.step(forward),
            () if forward => range.end,
            () => range.start,
        };
        self.set_cursor(target, select);
    }
}

#[cfg(feature = "cpu")]
pub struct LauncherState {
    pub open: bool,
    pub field: TextField,
    pub apps: Vec<DesktopApp>,
    pub matches: Vec<u32>,
    /// The fend answer for the current query, if any.
    pub calc_result: Option<String>,
    /// Index of the highlighted entry, which enter and shift+enter act on.
    pub selected: usize,
    /// Text waiting to be put on the system clipboard by the platform layer.
    pub pending_copy: Option<String>,
    calc: Context,
}

#[cfg(feature = "cpu")]
enum LauncherEntry<'a> {
    Answer(&'a str),
    App(&'a DesktopApp),
}

#[cfg(feature = "cpu")]
impl LauncherState {
    pub(crate) fn new(background: &Background, http: Agent) -> Self {
        let mut calc = Context::new();
        fetch_exchange_rates(background, http);
        calc.set_exchange_rate_handler_v2(ExchangeRates);
        start_scan(background);
        Self {
            open: false,
            field: TextField::default(),
            apps: Vec::new(),
            matches: Vec::new(),
            calc_result: None,
            selected: 0,
            pending_copy: None,
            calc,
        }
    }

    /// Opens or closes the launcher with a fresh query.
    pub fn toggle(&mut self) {
        self.open = !self.open;
        self.field.clear();
        self.refresh_matches();
    }

    pub const fn close(&mut self) {
        self.open = false;
    }

    /// Runs one edit against the search field, then re-runs the query.
    pub fn edit(&mut self, edit: impl FnOnce(&mut TextField)) {
        edit(&mut self.field);
        self.refresh_matches();
    }

    pub fn refresh_matches(&mut self) {
        let query = &self.field.text;
        self.calc_result = (query.len() >= 4)
            .then(|| fend_core::evaluate(query, &mut self.calc).ok())
            .flatten()
            .map(|result| result.get_main_result().to_owned())
            .filter(|result| !result.is_empty() && result != query);

        let query = query.to_lowercase();
        let visible = MAX_VISIBLE - usize::from(self.calc_result.is_some());
        let mut scored = self
            .apps
            .iter()
            .enumerate()
            .filter_map(|(index, app)| {
                let name = app.name.to_lowercase();
                name.contains(&query)
                    .then(|| (index as u32, name.starts_with(&query)))
            })
            .collect::<Vec<_>>();
        scored.sort_by_key(|&(_, prefix_match)| !prefix_match);
        self.matches = scored.into_iter().take(visible).map(|(index, _)| index).collect();
        self.selected = 0;
    }

    pub fn entry_count(&self) -> usize {
        usize::from(self.calc_result.is_some()) + self.matches.len()
    }

    fn entry(&self, row: usize) -> Option<LauncherEntry<'_>> {
        if let Some(answer) = self.calc_result.as_deref() {
            if row == 0 {
                return Some(LauncherEntry::Answer(answer));
            }
            row.checked_sub(1)
        } else {
            Some(row)
        }
        .and_then(|row| self.matches.get(row))
        .and_then(|&app| self.apps.get(app as usize))
        .map(LauncherEntry::App)
    }

    /// Moves the highlight by `delta` rows, stopping at either end.
    pub fn move_selection(&mut self, delta: i32) {
        self.selected = self
            .selected
            .saturating_add_signed(delta as isize)
            .min(self.entry_count().saturating_sub(1));
    }

    /// Runs row `index`'s action — its alternative one when `alternate` is set — then dismisses.
    pub fn activate(&mut self, index: usize, alternate: bool) {
        match self.entry(index) {
            Some(LauncherEntry::App(app)) => Platform::spawn(
                app.action
                    .as_ref()
                    .filter(|_| alternate)
                    .map_or(&app.exec, |(_, exec)| exec),
            ),
            Some(LauncherEntry::Answer(answer)) => self.pending_copy = Some(answer.to_owned()),
            None => return,
        }
        self.open = false;
        self.field.clear();
        self.refresh_matches();
    }
}

/// Currency rates relative to USD, fetched once and read by fend for currency conversions.
#[cfg(feature = "cpu")]
static EXCHANGE_RATES: OnceLock<HashMap<String, f64>> = OnceLock::new();

#[cfg(feature = "cpu")]
struct ExchangeRates;

#[cfg(feature = "cpu")]
impl fend_core::ExchangeRateFnV2 for ExchangeRates {
    fn relative_to_base_currency(
        &self,
        currency: &str,
        _options: &fend_core::ExchangeRateFnV2Options,
    ) -> Result<f64, Box<dyn Error + Send + Sync>> {
        EXCHANGE_RATES
            .get()
            .and_then(|rates| rates.get(currency))
            .copied()
            .ok_or_else(|| "exchange rates not loaded yet".into())
    }
}

#[cfg(feature = "cpu")]
fn fetch_exchange_rates(background: &Background, http: Agent) {
    background.run(move || {
        if let Ok(mut response) = http.get("https://open.er-api.com/v6/latest/USD").call()
            && let Ok(body) = response.body_mut().read_json::<CurrencyRates>()
        {
            let _ = EXCHANGE_RATES.set(body.rates);
        }
    });
}

#[cfg(feature = "cpu")]
#[derive(serde::Deserialize)]
struct CurrencyRates {
    rates: HashMap<String, f64>,
}

/// The panel, which draws the search field on itself, or one calculator/app row below it.
#[isthmus::data]
#[derive(Default)]
pub struct LauncherRow {
    pub y: f32,
    /// Icon texture layer, or one of the sentinels above.
    pub icon: i32,
    /// Search caret x within the panel, and its blink alpha.
    pub caret: Vec2,
    /// Highlighted span of the search text, empty when nothing is selected.
    pub selection: Vec2,
    /// Center x and half width of the enter and shift+enter badges; a zero width hides one.
    pub badges: [Vec2; 2],
    /// Primary text, secondary text, then the two action labels.
    pub lines: [text::Line; 4],
}

#[derive(isthmus::Varyings)]
pub struct Varyings {
    pub pixel: Vec2,
    #[gpu(flat)]
    pub row_idx: u32,
}

#[isthmus::pass]
pub struct LauncherPass {
    rows: isthmus::Instances<Self>,
    icons: SampledTexture<Texture2DArray>,
}

#[isthmus::pass]
impl LauncherPass {
    pub fn new(passes: &Passes<'_>, text: &text::Renderer) -> Self {
        let icons = passes.sampled_texture::<Texture2DArray>(
            "Launcher Icons",
            Extent3d {
                width: ICON_PX,
                height: ICON_PX,
                depth_or_array_layers: MAX_ICON_SLOTS as u32,
            },
            FilterableFloatFormat::Rgba8Unorm,
        );
        let sampler = passes.filtering_sampler("Launcher Icon Sampler");
        let (placed_glyphs, glyphs, edges) = text.resources();
        Self {
            rows: passes.instances_with_capacity(
                (icons.view(), &sampler, placed_glyphs, glyphs, edges),
                MAX_VISIBLE + 2,
            ),
            icons,
        }
    }

    /// Uploads a decoded `ICON_PX`×`ICON_PX` RGBA icon into a fixed texture layer.
    pub fn write_icon(&self, layer: u32, pixels: &[u8]) {
        if let Err(error) = self.icons.write([0, 0, layer], [ICON_PX; 2], pixels) {
            warn!(%error, layer, "Failed to upload app icon");
        }
    }

    /// Draws the panel and its search field, then the rows beneath it.
    pub fn update(
        &mut self,
        text: &mut text::Renderer,
        launcher: &mut LauncherState,
        frame: &mut Frame,
    ) {
        self.rows.clear();
        if !launcher.open {
            return;
        }
        let (origin, size) = background_bounds(frame.shared);
        let screen = frame.shared.screen_size;
        frame
            .interaction
            .input_region(Rect::new(0.0, 0.0, screen.x, screen.y));
        let panel = Rect::new(origin.x, origin.y, origin.x + size.x, origin.y + size.y);

        let header = header_height(frame.shared);
        let (left, right) = (PADDING + 34.0, size.x - PADDING);
        let field = &mut launcher.field;
        if field.touched {
            field.touched = false;
            field.blink_start = frame.shared.time;
        }
        let mut lines = [text::Line::default(); 4];
        lines[0] = if field.text.is_empty() {
            text.left("Search anything…", SEARCH_STYLE, header * 0.5, left)
                .with_color(MUTED_COLOR)
        } else {
            let query = text.shape(&field.text, SEARCH_STYLE);
            text.place_visible(&query, vec2(left, header * 0.5), left..right)
        };
        // Long queries are clipped rather than scrolled, so every offset maps straight to an x.
        let at = |offset: usize| (left + text.width(&field.text[..offset], SEARCH_STYLE)).min(right);
        let selection = field.selection();
        let blink = ((frame.shared.time - field.blink_start) * 1.4).fract();
        self.rows.push(LauncherRow {
            y: origin.y,
            icon: PANEL,
            caret: vec2(at(field.cursor), smoothstep(0.62, 0.5, blink)),
            selection: if selection.is_empty() {
                Vec2::ZERO
            } else {
                vec2(at(selection.start), at(selection.end))
            },
            lines,
            ..Default::default()
        });

        self.push_entries(text, launcher, frame, origin, frame.config.height);
        if frame.interaction.released() && !panel.contains(frame.interaction.pointer) {
            launcher.close();
        }
    }

    /// Lays out the calculator answer and matched app rows below the search field.
    fn push_entries(
        &mut self,
        text: &mut text::Renderer,
        launcher: &mut LauncherState,
        frame: &mut Frame,
        origin: Vec2,
        row_height: f32,
    ) {
        let (x, width) = row_bounds(frame.shared);
        let top = origin.y + header_height(frame.shared) + PADDING;
        let count = launcher.entry_count();
        let rect = |index: usize| {
            let y = top + index as f32 * (row_height + GAP);
            Rect::new(x, y, x + width, y + row_height)
        };
        // Resolve the highlight up front, so hover and the arrow keys agree across every row.
        if let Some(index) = (0..count).find(|&index| frame.interaction.contains(rect(index))) {
            launcher.selected = index;
        }
        let text_left = row_height * 0.5 + ICON_SIZE * 0.5 + GAP * 2.0;

        let mut activated = None;
        for index in 0..count {
            let row = rect(index);
            if frame.interaction.surface(row).clicked {
                activated = Some(index);
            }
            let (icon, name, detail, action, alternate) = match launcher.entry(index).unwrap() {
                LauncherEntry::App(app) => (
                    app.icon_layer,
                    app.name.as_str(),
                    app.comment.as_str(),
                    "Open",
                    app.action.as_ref().map(|(label, _)| label.as_str()),
                ),
                LauncherEntry::Answer(answer) => (CALCULATOR_ICON, answer, "", "Copy", None),
            };

            // Only the highlighted row spells out what enter and shift+enter would do.
            let mut badges = [Vec2::ZERO; 2];
            let mut lines = [text::Line::default(); 4];
            let mut edge = width - GAP * 2.0;
            if launcher.selected == index {
                for (slot, label) in [Some(action), alternate].into_iter().enumerate() {
                    let Some(label) = label else { continue };
                    let badge_width = BADGE_WIDTHS[slot];
                    badges[slot] = vec2(edge - badge_width * 0.5, badge_width * 0.5);
                    edge -= badge_width + GAP;
                    lines[2 + slot] = text
                        .right(label, DETAIL_STYLE, row_height * 0.5, edge)
                        .with_color(MUTED_COLOR);
                    edge -= text.width(label, DETAIL_STYLE) + GAP * 2.0;
                }
            }

            let clip = text_left..edge.max(text_left);
            let (name_y, detail_y) = if detail.is_empty() {
                (row_height * 0.5, 0.0)
            } else {
                (row_height * 0.34, row_height * 0.68)
            };
            let shaped = text.shape(name, NAME_STYLE);
            lines[0] = text.place_visible(&shaped, vec2(text_left, name_y), clip.clone());
            if !detail.is_empty() {
                let shaped = text.shape(detail, DETAIL_STYLE);
                lines[1] = text
                    .place_visible(&shaped, vec2(text_left, detail_y), clip)
                    .with_color(DETAIL_COLOR);
            }

            self.rows.push(LauncherRow {
                y: row.y0,
                icon,
                badges,
                lines,
                ..Default::default()
            });
        }
        if let Some(index) = activated {
            launcher.activate(index, false);
        }
    }

    #[gpu]
    pub fn vertex(
        #[gpu(vertex_index)] vertex: u32,
        #[gpu(instance_index)] instance: u32,
        #[gpu(shared)] frame: FrameData,
        #[gpu(instance)] row: LauncherRow,
    ) -> Vertex<Varyings> {
        let (position, pixel) = if row.icon == PANEL {
            let (origin, size) = background_bounds(frame);
            let pixel = origin + quad_coord(vertex) * size;
            (pixel_to_ndc(pixel, frame.screen_size), pixel)
        } else {
            let (x, width) = row_bounds(frame);
            pill_vertex(vertex, frame, x, row.y, vec2(width, 0.0))
        };
        Vertex {
            position,
            varyings: Varyings {
                pixel,
                row_idx: instance,
            },
        }
    }

    #[gpu]
    pub fn fragment(
        Varyings { pixel, row_idx: _ }: Varyings,
        #[gpu(shared)] frame: FrameData,
        #[gpu(instance = row_idx as usize)] row: LauncherRow,
        #[gpu(resource)] icons: &Texture2DArray,
        #[gpu(resource)] sampler: &Sampler,
        #[gpu(resource)] placed_glyphs: &[text::PlacedGlyph],
        #[gpu(resource)] glyphs: &[text::Glyph],
        #[gpu(resource)] edges: &[text::Edge],
    ) -> Vec4 {
        if row.icon == PANEL {
            let (origin, size) = background_bounds(frame);
            let mask = fill(sd_rounded_box(pixel - origin - size * 0.5, size * 0.5, 16.0));
            if mask <= 0.0 {
                kill();
            }
            // The panel also draws the search field across its header strip.
            let local = pixel - origin;
            let header = header_height(frame);
            let mut color = Vec3::splat(0.09).lerp(
                Vec3::splat(0.17),
                fill(sd_rounded_box(
                    local - vec2(size.x * 0.5, header - 0.5),
                    vec2(size.x * 0.5, 0.5),
                    0.0,
                )),
            );
            color = color.lerp(
                ICON_COLOR,
                magnifier_icon(local - vec2(PADDING + 11.0, header * 0.5)),
            );
            let span = row.selection.y - row.selection.x;
            let highlight = fill(sd_rounded_box(
                local - vec2((row.selection.x + row.selection.y) * 0.5, header * 0.5),
                vec2(span * 0.5, 13.0),
                3.0,
            ));
            color = color.lerp(vec3(0.24, 0.28, 0.52), highlight * presence(span));
            let caret = fill(sd_rounded_box(
                local - vec2(row.caret.x, header * 0.5),
                vec2(0.9, 12.0),
                0.9,
            ));
            color = color.lerp(text::COLOR, caret * row.caret.y);
            let query = row.lines[0];
            let alpha = text::line_alpha(query, placed_glyphs, glyphs, edges, local);
            return ((color.lerp(query.color.to_vec3(), alpha)) * mask).extend(mask);
        }

        let (x, width) = row_bounds(frame);
        let (interaction, local, size, surface) = pill_fragment(pixel, frame, x, row.y, width);
        let (dist, mask, alpha) = interaction.surface(surface);
        if alpha <= 1.0 / 1024.0 {
            kill();
        }
        let refracted = interaction.refract(local, size, dist) * size;
        // Only the highlighted row carries badges, so they double as its highlight flag.
        let mut color = Vec3::splat(0.15)
            .lerp(Vec3::splat(0.235), presence(row.badges[0].y))
            .lerp(Vec3::splat(0.3), (interaction.bulge(surface) / 8.0).min(1.0));

        let icon_point = local - Vec2::splat(size.y * 0.5);
        if row.icon == CALCULATOR_ICON {
            let icon = calculator_icon(icon_point);
            color = color.lerp(icon.truncate(), icon.w);
        } else if row.icon >= 0 && icon_point.abs().max_element() < ICON_SIZE * 0.5 {
            let uv = icon_point / ICON_SIZE + 0.5;
            let tex = icons.sample(*sampler, uv.extend(row.icon as f32));
            color = color.lerp(tex.truncate(), tex.w);
        }

        let mut index = 0;
        while index < 2 {
            let badge = row.badges[index];
            let key = action_badge(refracted - vec2(badge.x, size.y * 0.5), badge.y, index == 1);
            color = color.lerp(key.truncate(), key.w);
            index += 1;
        }

        let mut index = 0;
        while index < 4 {
            let line = row.lines[index];
            let text_alpha = text::line_alpha(line, placed_glyphs, glyphs, edges, refracted);
            color = color.lerp(line.color.to_vec3(), text_alpha);
            index += 1;
        }

        (color * mask).extend(alpha)
    }
}

/// Scans installed apps and decodes their icons on a background thread, then applies the result.
#[cfg(feature = "cpu")]
fn start_scan(background: &Background) {
    background.submit(|| {
        let mut apps = Platform::desktop_apps();
        apps.sort_by_key(|app| app.name.to_lowercase());
        let mut icon_writes = Vec::new();
        for (index, app) in apps.iter_mut().enumerate().take(MAX_ICON_SLOTS) {
            let Some(path) = app.icon_path.as_deref() else {
                continue;
            };
            if let Some(pixels) = load_icon_pixels(path) {
                icon_writes.push((index as u32, pixels));
                app.icon_layer = index as i32;
            } else {
                warn!(?path, "Failed to decode app icon");
            }
        }
        Box::new(move |app| {
            let passes = app.render.program().passes_mut();
            for (layer, pixels) in &icon_writes {
                passes.launcher.write_icon(*layer, pixels);
            }
            app.launcher.apps = apps;
            app.launcher.refresh_matches();
        })
    });
}

/// Rasterizes an icon to `ICON_PX` square, straight-alpha RGBA.
#[cfg(feature = "cpu")]
fn load_icon_pixels(path: &Path) -> Option<Vec<u8>> {
    if path.extension().is_some_and(|extension| extension == "svg") {
        let tree = Tree::from_data(&fs::read(path).ok()?, &usvg::Options::default()).ok()?;
        let mut pixmap = Pixmap::new(ICON_PX, ICON_PX)?;
        let source = tree.size();
        let square = ICON_PX as f32;
        let fit = Transform::from_scale(square / source.width(), square / source.height());
        render(&tree, fit, &mut pixmap.as_mut());
        return Some(pixmap.take_demultiplied());
    }
    let raster = image::open(path)
        .ok()?
        .resize_to_fill(ICON_PX, ICON_PX, FilterType::Triangle);
    Some(raster.into_rgba8().into_raw())
}
