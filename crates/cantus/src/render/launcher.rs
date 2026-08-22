use crate::render::{
    GAP, Globals, PADDING, TEXT_COLOR,
    sdf::{Pill, VISIBLE_ALPHA, fill, fill_rounded_box, presence, ripple_flash, sd_rounded_box, segment_distance, stroke},
    text::{self, TextStyle},
};
use isthmus::{
    ImageHandle, Quad,
    float::FloatExt,
    glam::{Vec2, Vec3, Vec4, vec2, vec3},
    spirv_std::arch::kill,
};

#[cfg(not(target_arch = "spirv"))]
use {
    crate::{
        app::{
            Background,
            config::SearchProvider,
            platform::{Current as Platform, DesktopApp, Platform as _},
            update,
        },
        render::Frame,
    },
    fend_core::Context,
    image::imageops::FilterType,
    isthmus::{ImageArray, Rect},
    reqwest::{Client, Url},
    resvg::{
        render,
        tiny_skia::{Pixmap, Transform},
        usvg::{self, Tree},
    },
    std::{collections::HashMap, error::Error, fs, ops::Range, path::Path, sync::OnceLock},
    tokio::task::spawn_blocking,
    tracing::warn,
};

const PANEL_WIDTH: f32 = 520.0;
pub(crate) const BACKGROUND_RADIUS: i32 = 16;
/// Matched-app/calculator rows shown below the search bar.
pub(crate) const MAX_VISIBLE: usize = 8;

/// Side of the square icon tile at the left of every row.
const ICON_SIZE: f32 = 32.0;
const BADGE_HEIGHT: f32 = 21.0;
/// Icons, badge outlines and the magnifier all share one grey.
const ICON_COLOR: Vec3 = Vec3::splat(0.58);
const ACCENT_COLOR: Vec3 = vec3(0.44, 0.40, 0.80);
const ENTER_BADGE_WIDTH: f32 = 27.0;
const ALTERNATE_BADGE_WIDTH: f32 = 42.0;
const ICON_PX: u32 = 48;
const MAX_ICON_SLOTS: usize = 192;
const SEARCH_STYLE: TextStyle = TextStyle::new(18.0, 600.0);
const NAME_STYLE: TextStyle = TextStyle::new(16.0, 700.0);
const DETAIL_STYLE: TextStyle = TextStyle::new(13.0, 600.0);
const DETAIL_COLOR: Vec4 = Vec4::new(0.56, 0.63, 0.86, 1.0);
const MUTED_COLOR: Vec4 = Vec4::new(0.52, 0.55, 0.64, 1.0);
const IMAGE_ICON: u32 = 0;
const CALCULATOR_ICON: u32 = 1;
const SEARCH_ICON: u32 = 2;
const CALCULATOR_ICON_CODE: i32 = -2;
const SEARCH_ICON_CODE: i32 = -1;

/// Height of the search field, which sits flush at the top of the panel instead of in a pill.
fn header_height(frame: &Globals) -> f32 {
    frame.panel_height + PADDING
}

/// Coverage of a magnifying glass centered on the origin.
fn magnifier_icon(point: Vec2) -> f32 {
    let ring = stroke(point.length() - 6.2, 1.05);
    let handle = stroke(segment_distance(point, vec2(4.6, 4.6), vec2(8.8, 8.8)), 1.05);
    ring.max(handle)
}

/// Straight color and coverage of the calculator badge shown beside a fend answer.
fn calculator_icon(point: Vec2) -> Vec4 {
    let badge = fill_rounded_box(point, Vec2::splat(13.0), 9.0);
    let bar = |offset: f32| fill_rounded_box(point - vec2(0.0, offset), vec2(5.4, 1.1), 1.1);
    let equals = bar(-3.1).max(bar(3.1));
    ACCENT_COLOR.lerp(Vec3::splat(0.96), equals).extend(badge.max(equals * badge))
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
    let color = Vec3::splat(0.27).lerp(ICON_COLOR, edge).lerp(TEXT_COLOR, glyph);
    color.extend(body.max(edge).max(glyph))
}

/// The search query with a caret and selection, edited like an ordinary text box.
#[cfg(not(target_arch = "spirv"))]
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

#[cfg(not(target_arch = "spirv"))]
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

#[derive(Clone, Copy)]
pub(crate) enum LauncherKey {
    Escape,
    Activate,
    Up,
    Down,
    Backspace,
    Delete,
    Left,
    Right,
    Home,
    End,
    SelectAll,
    Copy,
    Cut,
}

#[cfg(not(target_arch = "spirv"))]
pub struct LauncherState {
    pub open: bool,
    pub field: TextField,
    pub matches: Vec<u32>,
    /// The fend answer for the current query, if any.
    pub calc_result: Option<String>,
    /// Index of the highlighted entry, which enter and shift+enter act on.
    pub selected: usize,
    /// Text waiting to be put on the system clipboard by the platform layer.
    pub pending_copy: Option<String>,
    calc: Context,
    catalog: LauncherCatalog,
}

#[cfg(not(target_arch = "spirv"))]
struct SearchEngine {
    config: SearchProvider,
    label: String,
    icon_index: i32,
}

#[cfg(not(target_arch = "spirv"))]
struct LauncherCatalog {
    apps: Vec<DesktopApp>,
    providers: Vec<SearchEngine>,
    pending_icons: Vec<(usize, Vec<u8>)>,
    icon_handles: Vec<ImageHandle>,
    icons: Option<ImageArray>,
}

#[cfg(not(target_arch = "spirv"))]
enum LauncherEntry<'a> {
    Answer(&'a str),
    App(&'a DesktopApp),
    Search(&'a SearchEngine),
}

#[cfg(not(target_arch = "spirv"))]
struct EntryView<'a> {
    icon: i32,
    name: &'a str,
    detail: &'a str,
    action: &'static str,
    alternate: Option<&'a str>,
}

#[cfg(not(target_arch = "spirv"))]
impl<'a> LauncherEntry<'a> {
    fn view(self, search: &'a str) -> EntryView<'a> {
        match self {
            Self::App(app) => EntryView {
                icon: app.icon_index,
                name: &app.name,
                detail: &app.comment,
                action: "Open",
                alternate: app.action.as_ref().map(|(label, _)| label.as_str()),
            },
            Self::Answer(answer) => EntryView {
                icon: CALCULATOR_ICON_CODE,
                name: answer,
                detail: "",
                action: "Copy",
                alternate: None,
            },
            Self::Search(engine) => EntryView {
                icon: engine.icon_index,
                name: &engine.label,
                detail: search,
                action: "Search",
                alternate: None,
            },
        }
    }
}

#[cfg(not(target_arch = "spirv"))]
impl LauncherState {
    pub(crate) fn new(background: &Background, http: &Client, providers: impl IntoIterator<Item = SearchProvider>) -> Self {
        let mut calc = Context::new();
        fetch_exchange_rates(background, http.clone());
        calc.set_exchange_rate_handler_v2(ExchangeRates);
        let providers = providers
            .into_iter()
            .map(|config| SearchEngine {
                label: format!("Search with {}", config.name),
                config,
                icon_index: SEARCH_ICON_CODE,
            })
            .collect::<Vec<_>>();
        start_scan(background, http, &providers);
        Self {
            open: false,
            field: TextField::default(),
            matches: Vec::new(),
            calc_result: None,
            selected: 0,
            pending_copy: None,
            calc,
            catalog: LauncherCatalog {
                apps: Vec::new(),
                providers,
                pending_icons: Vec::new(),
                icon_handles: vec![ImageHandle::NONE; MAX_ICON_SLOTS],
                icons: None,
            },
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

    pub(crate) fn key(&mut self, key: LauncherKey, shift: bool) {
        match key {
            LauncherKey::Escape => self.close(),
            LauncherKey::Activate => self.activate(self.selected, shift),
            LauncherKey::Up => self.move_selection(-1),
            LauncherKey::Down => self.move_selection(1),
            LauncherKey::Backspace => self.edit(|field| field.erase(false)),
            LauncherKey::Delete => self.edit(|field| field.erase(true)),
            LauncherKey::Left => self.field.move_cursor(false, shift),
            LauncherKey::Right => self.field.move_cursor(true, shift),
            LauncherKey::Home => self.field.set_cursor(0, shift),
            LauncherKey::End => self.field.set_cursor(self.field.text.len(), shift),
            LauncherKey::SelectAll => self.field.select_all(),
            LauncherKey::Copy | LauncherKey::Cut => {
                self.pending_copy = Some(self.field.selected_text().to_owned());
                if matches!(key, LauncherKey::Cut) {
                    self.edit(|field| {
                        field.delete_selection();
                    });
                }
            }
        }
    }

    pub fn refresh_matches(&mut self) {
        let (provider, query) = self.search_query();
        let explicit_search = provider.is_some();
        let query = query.to_owned();
        self.calc_result = (!explicit_search && query.len() >= 4)
            .then(|| fend_core::evaluate(&query, &mut self.calc).ok())
            .flatten()
            .map(|result| result.get_main_result().to_owned())
            .filter(|result| !result.is_empty() && result != &query);

        let lower_query = query.to_lowercase();
        let has_search = !self.catalog.providers.is_empty() && (!lower_query.is_empty() || explicit_search);
        let visible = MAX_VISIBLE - usize::from(self.calc_result.is_some()) - usize::from(has_search);
        let mut scored = self
            .catalog
            .apps
            .iter()
            .enumerate()
            .filter(|_| !explicit_search)
            .filter_map(|(index, app)| {
                let name = app.name.to_lowercase();
                name.contains(&lower_query).then(|| (index as u32, name.starts_with(&lower_query)))
            })
            .collect::<Vec<_>>();
        scored.sort_by_key(|&(_, prefix_match)| !prefix_match);
        self.matches = scored.into_iter().take(visible).map(|(index, _)| index).collect();
        self.selected = 0;
    }

    pub fn entry_count(&self) -> usize {
        usize::from(self.calc_result.is_some()) + self.matches.len() + usize::from(self.search_provider().is_some())
    }

    fn entry(&self, row: usize) -> Option<LauncherEntry<'_>> {
        let mut row = row;
        if let Some(answer) = self.calc_result.as_deref() {
            if row == 0 {
                return Some(LauncherEntry::Answer(answer));
            }
            row -= 1;
        }
        self.matches
            .get(row)
            .and_then(|&app| self.catalog.apps.get(app as usize))
            .map(LauncherEntry::App)
            .or_else(|| (row == self.matches.len()).then(|| self.search_provider()).flatten().map(LauncherEntry::Search))
    }

    /// Moves the highlight by `delta` rows, stopping at either end.
    pub fn move_selection(&mut self, delta: i32) {
        self.selected = self.selected.saturating_add_signed(delta as isize).min(self.entry_count().saturating_sub(1));
    }

    /// Runs row `index`'s action — its alternative one when `alternate` is set — then dismisses.
    pub fn activate(&mut self, index: usize, alternate: bool) {
        match self.entry(index) {
            Some(LauncherEntry::App(app)) => Platform::spawn(app.action.as_ref().filter(|_| alternate).map_or(&app.exec, |(_, exec)| exec)),
            Some(LauncherEntry::Answer(answer)) => self.pending_copy = Some(answer.to_owned()),
            Some(LauncherEntry::Search(engine)) => {
                let terms = self.search_query().1;
                let encoded = form_urlencoded::byte_serialize(terms.as_bytes()).collect::<String>();
                Platform::open_url(&engine.config.url.replace("{searchTerms}", &encoded));
            }
            None => return,
        }
        self.open = false;
        self.field.clear();
        self.refresh_matches();
    }

    fn search_query(&self) -> (Option<usize>, &str) {
        let query = self.field.text.trim();
        self.catalog
            .providers
            .iter()
            .position(|provider| query == provider.config.alias || query.strip_prefix(&provider.config.alias).is_some_and(|rest| rest.starts_with(char::is_whitespace)))
            .map_or((None, query), |index| (Some(index), query[self.catalog.providers[index].config.alias.len()..].trim()))
    }

    fn search_provider(&self) -> Option<&SearchEngine> {
        let (provider, query) = self.search_query();
        (!query.is_empty() || provider.is_some())
            .then(|| self.catalog.providers.get(provider.unwrap_or(0)))
            .flatten()
    }

    pub(crate) fn bounds(&self, screen_size: Vec2, panel_height: f32) -> (Vec2, Vec2) {
        let rows = self.entry_count();
        let height = panel_height + PADDING * 3.0 + rows as f32 * panel_height + rows.saturating_sub(1) as f32 * GAP;
        let size = vec2(PANEL_WIDTH, height);
        ((screen_size - size) * 0.5, size)
    }
}

/// Currency rates relative to USD, fetched once and read by fend for currency conversions.
#[cfg(not(target_arch = "spirv"))]
static EXCHANGE_RATES: OnceLock<HashMap<String, f64>> = OnceLock::new();

struct ExchangeRates;

#[cfg(not(target_arch = "spirv"))]
impl fend_core::ExchangeRateFnV2 for ExchangeRates {
    fn relative_to_base_currency(&self, currency: &str, _options: &fend_core::ExchangeRateFnV2Options) -> Result<f64, Box<dyn Error + Send + Sync>> {
        EXCHANGE_RATES
            .get()
            .and_then(|rates| rates.get(currency))
            .copied()
            .ok_or_else(|| "exchange rates not loaded yet".into())
    }
}

#[cfg(not(target_arch = "spirv"))]
fn fetch_exchange_rates(background: &Background, http: Client) {
    background.spawn(async move {
        if let Ok(response) = http.get("https://open.er-api.com/v6/latest/USD").send().await.and_then(reqwest::Response::error_for_status)
            && let Ok(body) = response.json::<CurrencyRates>().await
        {
            let _ = EXCHANGE_RATES.set(body.rates);
        }
        None
    });
}

#[cfg(not(target_arch = "spirv"))]
#[derive(serde::Deserialize)]
struct CurrencyRates {
    rates: HashMap<String, f64>,
}

#[isthmus::paint]
impl LauncherState {
    /// Draws the panel and its search field, then the rows beneath it.
    pub fn show(&mut self, frame: &mut Frame) {
        if self.catalog.icons.is_none() {
            self.catalog.icons = Some(frame.image_array([ICON_PX; 2], MAX_ICON_SLOTS));
        }
        let Some(icons) = self.catalog.icons.as_mut() else {
            return;
        };
        for (index, pixels) in self.catalog.pending_icons.drain(..) {
            if let Ok(handle) = icons.insert([ICON_PX; 2], pixels) {
                self.catalog.icon_handles[index] = handle;
            } else {
                warn!(index, "Failed to upload launcher icon");
            }
        }
        if !self.open {
            return;
        }
        let (origin, size) = self.bounds(frame.globals.info.screen_size, frame.globals.panel_height);
        frame.input_region(Rect::new(0.0, 0.0, frame.globals.info.screen_size.x, frame.globals.info.screen_size.y));
        let header = header_height(&frame.globals);
        let (left, right) = (PADDING + 34.0, size.x - PADDING);
        if self.field.touched {
            self.field.touched = false;
            self.field.blink_start = frame.globals.info.time;
        }
        let line = if self.field.text.is_empty() {
            frame.text("Search anything…", SEARCH_STYLE).left(vec2(left, header * 0.5)).with_color(MUTED_COLOR)
        } else {
            frame.text(&self.field.text, SEARCH_STYLE).visible(vec2(left, header * 0.5), left..right)
        };
        // Long queries are clipped rather than scrolled, so every offset maps straight to an x.
        let at = |offset: usize| (left + frame.measure_text(&self.field.text[..offset], SEARCH_STYLE)).min(right);
        let selection_range = self.field.selection();
        let blink = ((frame.globals.info.time - self.field.blink_start) * 1.4).fract();
        let caret = vec2(at(self.field.cursor), blink.smoothstep(0.62, 0.5));
        let selection = if selection_range.is_empty() {
            Vec2::ZERO
        } else {
            vec2(at(selection_range.start), at(selection_range.end))
        };
        let quad = Quad::from_min_max(origin, origin + size);
        // GPU: Launcher panel and search selection.
        frame.paint_quad(quad, |frame: Globals, fragment: isthmus::Fragment, size: Vec2, caret: Vec2, selection: Vec2| {
            let mask = fill_rounded_box(fragment.local, size * 0.5, BACKGROUND_RADIUS as f32);
            if mask <= 0.0 {
                kill();
            }
            let panel = fragment.local + size * 0.5;
            let header = header_height(&frame);
            let mut color = Vec3::splat(0.09).lerp(Vec3::splat(0.17), fill_rounded_box(panel - vec2(size.x * 0.5, header - 0.5), vec2(size.x * 0.5, 0.5), 0.0));
            color = color.lerp(color * 1.5 + 0.1, ripple_flash(fragment.pixel, &frame));
            color = color.lerp(ICON_COLOR, magnifier_icon(panel - vec2(PADDING + 11.0, header * 0.5)));

            let selection_width = selection.y - selection.x;
            let highlight = fill_rounded_box(panel - vec2(f32::midpoint(selection.x, selection.y), header * 0.5), vec2(selection_width * 0.5, 13.0), 3.0);
            color = color.lerp(vec3(0.24, 0.28, 0.52), highlight * presence(selection_width));
            let caret_mask = fill_rounded_box(panel - vec2(caret.x, header * 0.5), vec2(0.9, 12.0), 0.9);
            color = color.lerp(TEXT_COLOR, caret_mask * caret.y);

            let opacity = mask * 0.82;
            (color * opacity).extend(opacity)
        });
        let padding = line.size * 0.5 + 2.0;
        // GPU: Launcher query text.
        frame.paint_text(
            line.expanded(padding).translated(origin),
            |_frame: Globals, text: isthmus::TextFragment, origin: Vec2, size: Vec2| {
                let clip = fill_rounded_box(text.pixel - origin - size * 0.5, size * 0.5, BACKGROUND_RADIUS as f32);
                text.color(text.alpha() * clip)
            },
        );

        self.show_entries(frame, origin, size, frame.services.config.height);
    }

    /// Draws and interacts with calculator, application and search rows in visual order.
    fn show_entries(&mut self, frame: &mut Frame, origin: Vec2, size: Vec2, row_height: f32) {
        let (x, width) = (origin.x + PADDING, size.x - PADDING * 2.0);
        let mut y = origin.y + header_height(&frame.globals) + PADDING;
        let text_left = row_height * 0.5 + ICON_SIZE * 0.5 + GAP * 2.0;

        let mut activated = None;
        for index in 0..self.entry_count() {
            let pill = Pill::new(x, y, width);
            let response = frame.interact(pill.rect(row_height));
            if response.hovered() {
                self.selected = index;
            }
            if response.clicked() {
                activated = Some(index);
            }
            let entry = self.entry(index).unwrap().view(self.search_query().1);

            // Only the highlighted row spells out what enter and shift+enter would do.
            let mut edge = width - GAP * 2.0;
            let mut badge = |label: Option<&str>, width: f32| {
                let Some(label) = label.filter(|_| self.selected == index) else {
                    return (Vec2::ZERO, text::Line::default());
                };
                let badge = vec2(edge - width * 0.5, width * 0.5);
                edge -= width + GAP;
                let line = frame.text(label, DETAIL_STYLE).right(vec2(edge, row_height * 0.5)).with_color(MUTED_COLOR);
                edge -= frame.measure_text(label, DETAIL_STYLE) + GAP * 2.0;
                (badge, line)
            };
            let (enter_badge, action_line) = badge(Some(entry.action), ENTER_BADGE_WIDTH);
            let (alternate_badge, alternate_line) = badge(entry.alternate, ALTERNATE_BADGE_WIDTH);

            let clip = text_left..edge.max(text_left);
            let (name_y, detail_y) = if entry.detail.is_empty() {
                (row_height * 0.5, 0.0)
            } else {
                (row_height * 0.34, row_height * 0.68)
            };
            let name_line = frame.text(entry.name, NAME_STYLE).visible(vec2(text_left, name_y), clip.clone());
            let detail_line = if entry.detail.is_empty() {
                text::Line::default()
            } else {
                frame.text(entry.detail, DETAIL_STYLE).visible(vec2(text_left, detail_y), clip).with_color(DETAIL_COLOR)
            };

            let (image, icon_kind) = if entry.icon == CALCULATOR_ICON_CODE {
                (ImageHandle::NONE, CALCULATOR_ICON)
            } else if entry.icon == SEARCH_ICON_CODE {
                (ImageHandle::NONE, SEARCH_ICON)
            } else {
                (self.catalog.icon_handles[entry.icon as usize], IMAGE_ICON)
            };
            let quad = pill.quad(&frame.globals);
            // GPU: Launcher result row.
            frame.paint_quad(
                quad,
                |frame: Globals, fragment: isthmus::Fragment, pill: Pill, image: ImageHandle, icon_kind: u32, enter_badge: Vec2, alternate_badge: Vec2| {
                    let surface = pill.sample(fragment.pixel, &frame);
                    if surface.alpha <= VISIBLE_ALPHA {
                        kill();
                    }
                    // Only the highlighted row carries an enter badge, so it doubles as its highlight flag.
                    let mut color = Vec3::splat(0.15)
                        .lerp(Vec3::splat(0.235), presence(enter_badge.y))
                        .lerp(Vec3::splat(0.3), (surface.bulge() / 8.0).min(1.0));

                    let icon_point = surface.local - Vec2::splat(surface.size.y * 0.5);
                    if icon_kind == CALCULATOR_ICON {
                        let calculator = calculator_icon(icon_point);
                        color = color.lerp(calculator.truncate(), calculator.w);
                    } else if icon_kind == IMAGE_ICON
                        && icon_point.abs().max_element() < ICON_SIZE * 0.5
                        && let Some(texture) = fragment.sample_image(image, icon_point / ICON_SIZE + 0.5)
                    {
                        color = color.lerp(texture.truncate(), texture.w);
                    }

                    let enter = action_badge(surface.refracted - vec2(enter_badge.x, surface.size.y * 0.5), enter_badge.y, false);
                    color = color.lerp(enter.truncate(), enter.w);
                    let alternate = action_badge(surface.refracted - vec2(alternate_badge.x, surface.size.y * 0.5), alternate_badge.y, true);
                    color = color.lerp(alternate.truncate(), alternate.w);

                    surface.color(color)
                },
            );

            let origin = vec2(x, y);
            for line in [name_line, detail_line, action_line, alternate_line] {
                if !line.is_empty() {
                    // GPU: Launcher result text.
                    frame.paint_text(
                        line.expanded(line.size * 0.5 + 2.0).translated(origin),
                        |frame: Globals, text: isthmus::TextFragment, pill: Pill| {
                            let surface = pill.sample(text.pixel, &frame);
                            text.color(text.alpha() * surface.mask)
                        },
                    );
                }
            }
            y += row_height + GAP;
        }
        if let Some(index) = activated {
            self.activate(index, false);
        }
    }
}

/// Scans installed apps and decodes their icons on a background thread, then applies the result.
#[cfg(not(target_arch = "spirv"))]
fn start_scan(background: &Background, http: &Client, providers: &[SearchEngine]) {
    let provider_icon_count = providers.len().min(MAX_ICON_SLOTS);
    let provider_icons = providers
        .iter()
        .take(provider_icon_count)
        .enumerate()
        .filter_map(|(index, provider)| {
            let url = Url::parse(&provider.config.url.replace("{searchTerms}", "")).ok()?;
            Some((index, url))
        })
        .collect::<Vec<_>>();
    let app_slots = MAX_ICON_SLOTS - provider_icon_count;
    background.spawn(async move {
        let (apps, icon_writes) = spawn_blocking(move || {
            let mut apps = Platform::desktop_apps();
            apps.sort_by_key(|app| app.name.to_lowercase());
            let mut icon_writes = Vec::new();
            for (index, app) in apps.iter_mut().enumerate().take(app_slots) {
                let Some(path) = app.icon_path.as_deref() else {
                    continue;
                };
                if let Some(pixels) = load_icon_pixels(path) {
                    icon_writes.push((index, pixels));
                    app.icon_index = index as i32;
                } else {
                    warn!(?path, "Failed to decode app icon");
                }
            }
            (apps, icon_writes)
        })
        .await
        .unwrap_or_default();
        Some(update(move |app| {
            app.ui.launcher.catalog.pending_icons.extend(icon_writes);
            app.ui.launcher.catalog.apps = apps;
            app.ui.launcher.refresh_matches();
        }))
    });

    for (index, url) in provider_icons {
        let http = http.clone();
        background.spawn(async move {
            let pixels = fetch_favicon(&http, url).await?;
            let icon_index = app_slots + index;
            Some(update(move |app| {
                app.ui.launcher.catalog.pending_icons.push((icon_index, pixels));
                app.ui.launcher.catalog.providers[index].icon_index = icon_index as i32;
                app.ui.launcher.refresh_matches();
            }))
        });
    }
}

#[cfg(not(target_arch = "spirv"))]
async fn fetch_favicon(http: &Client, page: Url) -> Option<Vec<u8>> {
    let html = http.get(page.clone()).send().await.ok()?.text().await.ok()?;
    let advertised = html
        .split("<link")
        .skip(1)
        .filter_map(|tag| tag.split_once('>').map(|pair| pair.0))
        .find(|tag| attribute(tag, "rel").is_some_and(|rel| rel.contains("icon")))
        .and_then(|tag| attribute(tag, "href"))
        .and_then(|href| page.join(href).ok());
    let fallback = (|| {
        let mut labels = page.host_str()?.rsplit('.');
        let (suffix, name) = (labels.next()?, labels.next()?);
        Url::parse(&format!("https://{name}.{suffix}/favicon.ico")).ok()
    })();
    for icon in advertised.into_iter().chain(fallback) {
        let Some(bytes) = http.get(icon).send().await.ok().and_then(|response| response.error_for_status().ok()) else {
            continue;
        };
        let Ok(bytes) = bytes.bytes().await else { continue };
        if let Ok(Some(pixels)) = spawn_blocking(move || load_raster(&bytes)).await {
            return Some(pixels);
        }
    }
    None
}

fn attribute<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let value = tag.split_once(name)?.1.trim_start().strip_prefix('=')?.trim_start();
    match value.as_bytes().first()? {
        quote @ (b'\'' | b'"') => value[1..].split_once(char::from(*quote)).map(|pair| pair.0),
        _ => value.split_whitespace().next(),
    }
}

/// Rasterizes an icon to `ICON_PX` square, straight-alpha RGBA.
#[cfg(not(target_arch = "spirv"))]
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
    let raster = image::open(path).ok()?;
    Some(resize_icon(&raster))
}

#[cfg(not(target_arch = "spirv"))]
fn load_raster(bytes: &[u8]) -> Option<Vec<u8>> {
    if let Ok(image) = image::load_from_memory(bytes) {
        return Some(resize_icon(&image));
    }
    let tree = Tree::from_data(bytes, &usvg::Options::default()).ok()?;
    let mut pixmap = Pixmap::new(ICON_PX, ICON_PX)?;
    let source = tree.size();
    let fit = Transform::from_scale(ICON_PX as f32 / source.width(), ICON_PX as f32 / source.height());
    render(&tree, fit, &mut pixmap.as_mut());
    Some(pixmap.take_demultiplied())
}

#[cfg(not(target_arch = "spirv"))]
fn resize_icon(image: &image::DynamicImage) -> Vec<u8> {
    let raster = image.resize_to_fill(ICON_PX, ICON_PX, FilterType::Triangle);
    raster.into_rgba8().into_raw()
}
