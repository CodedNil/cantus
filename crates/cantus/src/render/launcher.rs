use crate::render::{
    Fragment, GAP, PADDING, TEXT_COLOR, TextFragment,
    sdf::{VISIBLE_ALPHA, fill, fill_rounded_box, pill_margin, presence, ripple_flash, ripple_light, sample_pill, sd_rounded_box, segment_distance, stroke},
};
use isthmus::{
    FloatExt, Image, Quad,
    glam::{Vec2, Vec3, Vec4, vec2, vec3},
    rgba,
    spirv_std::arch::kill,
    text::{self, TextStyle},
};

const PANEL_WIDTH: f32 = 520.0;
const ROW_HEIGHT: f32 = 50.0;
const HEADER_HEIGHT: f32 = ROW_HEIGHT + PADDING;
pub const BACKGROUND_RADIUS: i32 = 16;
/// Matched-app/calculator rows shown below the search bar.
pub const MAX_VISIBLE: usize = 8;

/// Side of the square icon tile at the left of every row.
const ICON_SIZE: f32 = 32.0;
const BADGE_HEIGHT: f32 = 21.0;
/// Icons, badge outlines and the magnifier all share one grey.
const ICON_COLOR: Vec3 = Vec3::splat(0.58);
const ACCENT_COLOR: Vec3 = vec3(0.44, 0.40, 0.80);
const ENTER_BADGE_WIDTH: f32 = 27.0;
const ALTERNATE_BADGE_WIDTH: f32 = 42.0;
const ICON_PX: u32 = 48;
const SEARCH_STYLE: TextStyle = TextStyle::new(18.0, 600.0);
const NAME_STYLE: TextStyle = TextStyle::new(16.0, 700.0);
const DETAIL_STYLE: TextStyle = TextStyle::new(13.0, 600.0);
const DETAIL_COLOR: Vec4 = Vec4::new(0.56, 0.63, 0.86, 1.0);
const MUTED_COLOR: Vec4 = Vec4::new(0.52, 0.55, 0.64, 1.0);
const CALCULATOR_ICON: u32 = 1;
const SEARCH_ICON: u32 = 2;

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

#[isthmus::paint]
mod host {
    use super::*;

    use crate::interaction::Rect;
    use crate::{
        app::{
            Background,
            config::SearchProvider,
            platform::{DesktopApp, Linux as Platform, Platform as _},
        },
        render::RenderContext,
    };
    use fend_core::Context;
    use image::imageops::FilterType;
    use reqwest::Client;
    use resvg::{
        render,
        tiny_skia::{Pixmap, Transform},
        usvg::{self, Tree},
    };
    use std::{
        collections::HashMap,
        error::Error,
        fs,
        ops::Range,
        path::Path,
        sync::{
            OnceLock,
            mpsc::{self, Receiver, Sender},
        },
    };
    use tokio::task::spawn_blocking;
    use tracing::warn;

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
    pub enum LauncherKey {
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

    pub struct LauncherState {
        pub open: bool,
        pub field: TextField,
        pub matches: Vec<usize>,
        /// The fend answer for the current query, if any.
        pub calc_result: Option<String>,
        /// Index of the highlighted entry, which enter and shift+enter act on.
        pub selected: usize,
        /// Text waiting to be put on the system clipboard by the platform layer.
        pub pending_copy: Option<String>,
        calc: Context,
        apps: Vec<DesktopApp>,
        providers: Vec<SearchEngine>,
        updates: Receiver<LauncherUpdate>,
    }

    struct SearchEngine {
        config: SearchProvider,
        label: String,
        icon: Option<Image>,
    }

    enum LauncherUpdate {
        Apps(Vec<DesktopApp>),
        ProviderIcon(usize, Image),
    }

    enum LauncherEntry<'a> {
        Answer(&'a str),
        App(&'a DesktopApp),
        Search(&'a SearchEngine),
    }

    struct EntryView<'a> {
        icon: EntryIcon<'a>,
        name: &'a str,
        detail: &'a str,
        action: &'static str,
        alternate: Option<&'a str>,
    }

    enum EntryIcon<'a> {
        Image(&'a Image),
        Calculator,
        Search,
    }

    impl<'a> LauncherEntry<'a> {
        fn view(self, search: &'a str) -> EntryView<'a> {
            match self {
                Self::App(app) => EntryView {
                    icon: app.icon.as_ref().map_or(EntryIcon::Search, EntryIcon::Image),
                    name: &app.name,
                    detail: &app.comment,
                    action: "Open",
                    alternate: app.action.as_ref().map(|(label, _)| label.as_str()),
                },
                Self::Answer(answer) => EntryView {
                    icon: EntryIcon::Calculator,
                    name: answer,
                    detail: "",
                    action: "Copy",
                    alternate: None,
                },
                Self::Search(engine) => EntryView {
                    icon: engine.icon.as_ref().map_or(EntryIcon::Search, EntryIcon::Image),
                    name: &engine.label,
                    detail: search,
                    action: "Search",
                    alternate: None,
                },
            }
        }
    }

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
                    icon: None,
                })
                .collect::<Vec<_>>();
            let (updates, inbox) = mpsc::channel();
            start_scan(background, http, &providers, &updates);
            Self {
                open: false,
                field: TextField::default(),
                matches: Vec::new(),
                calc_result: None,
                selected: 0,
                pending_copy: None,
                calc,
                apps: Vec::new(),
                providers,
                updates: inbox,
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
            let has_search = !self.providers.is_empty() && (!lower_query.is_empty() || explicit_search);
            let visible = MAX_VISIBLE - usize::from(self.calc_result.is_some()) - usize::from(has_search);
            let mut scored = self
                .apps
                .iter()
                .enumerate()
                .filter(|_| !explicit_search)
                .filter_map(|(index, app)| {
                    let name = app.name.to_lowercase();
                    name.contains(&lower_query).then(|| (index, name.starts_with(&lower_query)))
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
                .and_then(|&app| self.apps.get(app))
                .map(LauncherEntry::App)
                .or_else(|| self.search_provider().filter(|_| row == self.matches.len()).map(LauncherEntry::Search))
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
            self.providers
                .iter()
                .position(|provider| query == provider.config.alias || query.strip_prefix(&provider.config.alias).is_some_and(|rest| rest.starts_with(char::is_whitespace)))
                .map_or((None, query), |index| (Some(index), query[self.providers[index].config.alias.len()..].trim()))
        }

        fn search_provider(&self) -> Option<&SearchEngine> {
            let (provider, query) = self.search_query();
            self.providers.get(provider.unwrap_or_default()).filter(|_| provider.is_some() || !query.is_empty())
        }

        pub(crate) fn bounds(&self, screen_size: Vec2) -> (Vec2, Vec2) {
            let rows = self.entry_count();
            let height = HEADER_HEIGHT + PADDING * 2.0 + rows as f32 * ROW_HEIGHT + rows.saturating_sub(1) as f32 * GAP;
            let size = vec2(PANEL_WIDTH, height);
            ((screen_size - size) * 0.5, size)
        }
    }

    /// Currency rates relative to USD, fetched once and read by fend for currency conversions.
    static EXCHANGE_RATES: OnceLock<HashMap<String, f64>> = OnceLock::new();

    struct ExchangeRates;

    impl fend_core::ExchangeRateFnV2 for ExchangeRates {
        fn relative_to_base_currency(&self, currency: &str, _options: &fend_core::ExchangeRateFnV2Options) -> Result<f64, Box<dyn Error + Send + Sync>> {
            EXCHANGE_RATES
                .get()
                .and_then(|rates| rates.get(currency))
                .copied()
                .ok_or_else(|| "exchange rates not loaded yet".into())
        }
    }

    fn fetch_exchange_rates(background: &Background, http: Client) {
        background.spawn_update(async move {
            if let Ok(response) = http.get("https://open.er-api.com/v6/latest/USD").send().await.and_then(reqwest::Response::error_for_status)
                && let Ok(body) = response.json::<CurrencyRates>().await
            {
                let _ = EXCHANGE_RATES.set(body.rates);
            }
            None
        });
    }

    #[derive(serde::Deserialize)]
    struct CurrencyRates {
        rates: HashMap<String, f64>,
    }

    /// Scans installed apps and decodes their icons on a background thread, then applies the result.
    fn start_scan(background: &Background, http: &Client, providers: &[SearchEngine], updates: &Sender<LauncherUpdate>) {
        let app_updates = updates.clone();
        background.spawn(async move {
            let apps = spawn_blocking(move || {
                let mut apps = Platform::desktop_apps();
                apps.sort_by_key(|app| app.name.to_lowercase());
                for app in &mut apps {
                    let Some(path) = app.icon_path.take() else {
                        continue;
                    };
                    if let Some(pixels) = load_icon_pixels(&path) {
                        app.icon = Some(Image::rgba8([ICON_PX; 2], pixels));
                    } else {
                        warn!(?path, "Failed to decode app icon");
                    }
                }
                apps
            })
            .await
            .unwrap_or_default();
            let _ = app_updates.send(LauncherUpdate::Apps(apps));
        });

        for (index, provider) in providers.iter().enumerate() {
            let icon = provider.config.icon.clone();
            let http = http.clone();
            let updates = updates.clone();
            background.spawn(async move {
                if let Some(pixels) = fetch_icon(&http, &icon).await {
                    let _ = updates.send(LauncherUpdate::ProviderIcon(index, Image::rgba8([ICON_PX; 2], pixels)));
                }
            });
        }
    }

    async fn fetch_icon(http: &Client, url: &str) -> Option<Vec<u8>> {
        let bytes = http.get(url).send().await.ok()?.error_for_status().ok()?.bytes().await.ok()?;
        spawn_blocking(move || load_raster(&bytes)).await.ok().flatten()
    }

    /// Rasterizes an icon to `ICON_PX` square, straight-alpha RGBA.
    fn load_icon_pixels(path: &Path) -> Option<Vec<u8>> {
        if path.extension().is_some_and(|extension| extension == "svg") {
            return rasterize_svg(&fs::read(path).ok()?);
        }
        image::open(path).ok().map(|image| resize_icon(&image))
    }

    fn load_raster(bytes: &[u8]) -> Option<Vec<u8>> {
        image::load_from_memory(bytes).map(|image| resize_icon(&image)).ok().or_else(|| rasterize_svg(bytes))
    }

    fn rasterize_svg(bytes: &[u8]) -> Option<Vec<u8>> {
        let tree = Tree::from_data(bytes, &usvg::Options::default()).ok()?;
        let mut pixmap = Pixmap::new(ICON_PX, ICON_PX)?;
        let source = tree.size();
        let fit = Transform::from_scale(ICON_PX as f32 / source.width(), ICON_PX as f32 / source.height());
        render(&tree, fit, &mut pixmap.as_mut());
        Some(pixmap.take_demultiplied())
    }

    fn resize_icon(image: &image::DynamicImage) -> Vec<u8> {
        let raster = image.resize_to_fill(ICON_PX, ICON_PX, FilterType::Triangle);
        raster.into_rgba8().into_raw()
    }
    /// The search query with a caret and selection, edited like an ordinary text box.
    impl LauncherState {
        /// Draws the panel and its search field, then the rows beneath it.
        pub fn show(&mut self, frame: &mut RenderContext) {
            let mut refreshed = false;
            while let Ok(update) = self.updates.try_recv() {
                match update {
                    LauncherUpdate::Apps(apps) => {
                        self.apps = apps;
                    }
                    LauncherUpdate::ProviderIcon(index, pixels) => {
                        self.providers[index].icon = Some(pixels);
                    }
                }
                refreshed = true;
            }
            if refreshed {
                self.refresh_matches();
            }
            if !self.open {
                return;
            }
            let (origin, size) = self.bounds(frame.paint.screen_size);
            frame.interaction.input_region(Rect::new(0.0, 0.0, frame.paint.screen_size.x, frame.paint.screen_size.y));
            let (left, right) = (PADDING + 34.0, size.x - PADDING);
            if self.field.touched {
                self.field.touched = false;
                self.field.blink_start = frame.paint.time;
            }
            let line = if self.field.text.is_empty() {
                frame
                    .paint
                    .text()
                    .line("Search anything…", SEARCH_STYLE)
                    .left(vec2(left, HEADER_HEIGHT * 0.5))
                    .with_color(MUTED_COLOR)
            } else {
                frame
                    .paint
                    .text()
                    .line(&self.field.text, SEARCH_STYLE)
                    .visible(vec2(left, HEADER_HEIGHT * 0.5), left..right)
            };
            let selection_range = self.field.selection();
            let blink = ((frame.paint.time - self.field.blink_start) * 1.4).fract();
            let (caret, selection) = {
                let text = frame.paint.text();
                // Long queries are clipped rather than scrolled, so every offset maps straight to an x.
                let at = |offset: usize| (left + text.width(&self.field.text[..offset], SEARCH_STYLE)).min(right);
                let caret = vec2(at(self.field.cursor), blink.smoothstep(0.62, 0.5));
                let selection = if selection_range.is_empty() {
                    Vec2::ZERO
                } else {
                    vec2(at(selection_range.start), at(selection_range.end))
                };
                (caret, selection)
            };
            let quad = Quad::from_min_max(origin, origin + size);
            // GPU: Launcher panel and search selection.
            frame.paint.paint_quad(quad, |fragment: Fragment, size: Vec2, caret: Vec2, selection: Vec2| {
                let mask = fill_rounded_box(fragment.local, size * 0.5, BACKGROUND_RADIUS as f32);
                if mask <= 0.0 {
                    kill();
                }
                let panel = fragment.local + size * 0.5;
                let mut color = Vec3::splat(0.09).lerp(
                    Vec3::splat(0.17),
                    fill_rounded_box(panel - vec2(size.x * 0.5, HEADER_HEIGHT - 0.5), vec2(size.x * 0.5, 0.5), 0.0),
                );
                color = ripple_light(color, ripple_flash(fragment.pixel, fragment.globals, fragment.time));
                color = color.lerp(ICON_COLOR, magnifier_icon(panel - vec2(PADDING + 11.0, HEADER_HEIGHT * 0.5)));

                let selection_width = selection.y - selection.x;
                let highlight = fill_rounded_box(
                    panel - vec2(f32::midpoint(selection.x, selection.y), HEADER_HEIGHT * 0.5),
                    vec2(selection_width * 0.5, 13.0),
                    3.0,
                );
                color = color.lerp(vec3(0.24, 0.28, 0.52), highlight * presence(selection_width));
                let caret_mask = fill_rounded_box(panel - vec2(caret.x, HEADER_HEIGHT * 0.5), vec2(0.9, 12.0), 0.9);
                color = color.lerp(TEXT_COLOR, caret_mask * caret.y);

                let opacity = mask * 0.82;
                rgba(color, opacity)
            });
            let padding = line.size * 0.5 + 2.0;
            // GPU: Launcher query text.
            frame
                .paint
                .paint_text(line.expanded(padding).translated(origin), |text: TextFragment, origin: Vec2, size: Vec2| {
                    let clip = fill_rounded_box(text.pixel - origin - size * 0.5, size * 0.5, BACKGROUND_RADIUS as f32);
                    text.color(text.alpha() * clip)
                });

            self.show_entries(frame, origin);
        }

        /// Draws and interacts with calculator, application and search rows in visual order.
        fn show_entries(&mut self, frame: &mut RenderContext, origin: Vec2) {
            let (x, width) = (origin.x + PADDING, PANEL_WIDTH - PADDING * 2.0);
            let mut y = origin.y + HEADER_HEIGHT + PADDING;
            let text_left = ROW_HEIGHT * 0.5 + ICON_SIZE * 0.5 + GAP * 2.0;

            let mut activated = None;
            for index in 0..self.entry_count() {
                let pill = Rect::new(x, y, x + width, y + ROW_HEIGHT);
                let response = frame.interaction.interact(pill);
                if response.hovered() {
                    self.selected = index;
                }
                if response.clicked() {
                    activated = Some(index);
                }
                let pill: Quad = pill.into();
                let entry = self.entry(index).unwrap().view(self.search_query().1);

                // Only the highlighted row spells out what enter and shift+enter would do.
                let mut edge = width - GAP * 2.0 - 4.0;
                let mut badge = |label: Option<&str>, width: f32| {
                    let Some(label) = label.filter(|_| self.selected == index) else {
                        return (Vec2::ZERO, text::Line::default());
                    };
                    let badge = vec2(edge - width * 0.5, width * 0.5);
                    edge -= width + GAP;
                    let line = frame.paint.text().line(label, DETAIL_STYLE).right(vec2(edge, ROW_HEIGHT * 0.5)).with_color(MUTED_COLOR);
                    edge -= frame.paint.text().width(label, DETAIL_STYLE) + GAP * 2.0;
                    (badge, line)
                };
                let (enter_badge, action_line) = badge(Some(entry.action), ENTER_BADGE_WIDTH);
                let (alternate_badge, alternate_line) = badge(entry.alternate, ALTERNATE_BADGE_WIDTH);

                let clip = text_left..edge.max(text_left);
                let (name_y, detail_y) = if entry.detail.is_empty() {
                    (ROW_HEIGHT * 0.5, 0.0)
                } else {
                    (ROW_HEIGHT * 0.34, ROW_HEIGHT * 0.68)
                };
                let name_line = frame.paint.text().line(entry.name, NAME_STYLE).visible(vec2(text_left, name_y), clip.clone());
                let detail_line = if entry.detail.is_empty() {
                    text::Line::default()
                } else {
                    frame
                        .paint
                        .text()
                        .line(entry.detail, DETAIL_STYLE)
                        .visible(vec2(text_left, detail_y), clip)
                        .with_color(DETAIL_COLOR)
                };

                let (image, icon_kind) = match entry.icon {
                    EntryIcon::Image(image) => (Some(image), 0),
                    EntryIcon::Calculator => (None, CALCULATOR_ICON),
                    EntryIcon::Search => (None, SEARCH_ICON),
                };
                let quad = pill.expanded(pill_margin(frame.globals(), frame.paint.time));
                // GPU: Launcher result row.
                frame
                    .paint
                    .paint_quad(quad, |fragment: Fragment, pill: Quad, icon_kind: u32, enter_badge: Vec2, alternate_badge: Vec2| {
                        let surface = sample_pill(pill, fragment.pixel, fragment.globals, fragment.time);
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
                        } else if icon_kind == SEARCH_ICON {
                            color = color.lerp(ICON_COLOR, magnifier_icon(icon_point));
                        }

                        let enter = action_badge(surface.refracted - vec2(enter_badge.x, surface.size.y * 0.5), enter_badge.y, false);
                        color = color.lerp(enter.truncate(), enter.w);
                        let alternate = action_badge(surface.refracted - vec2(alternate_badge.x, surface.size.y * 0.5), alternate_badge.y, true);
                        color = color.lerp(alternate.truncate(), alternate.w);

                        surface.color(color)
                    });
                if let Some(image) = image {
                    let icon = Quad::new(pill.center - vec2((pill.size.x - pill.size.y) * 0.5, 0.0), Vec2::splat(ICON_SIZE), Vec2::X);
                    // GPU: Launcher result image.
                    frame.paint.paint_quad(icon, |fragment: Fragment, image: Image| {
                        let texture = image.sample(fragment.uv);
                        rgba(texture.truncate(), texture.w)
                    });
                }

                let origin = vec2(x, y);
                for line in [name_line, detail_line, action_line, alternate_line] {
                    if !line.is_empty() {
                        // GPU: Launcher result text.
                        frame
                            .paint
                            .paint_text(line.expanded(line.size * 0.5 + 2.0).translated(origin), |text: TextFragment, pill: Quad| {
                                let surface = sample_pill(pill, text.pixel, text.globals, text.time);
                                text.color(text.alpha() * surface.mask)
                            });
                    }
                }
                y += ROW_HEIGHT + GAP;
            }
            if let Some(index) = activated {
                self.activate(index, false);
            }
        }
    }
}
