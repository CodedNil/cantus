use egui::{
    Align, CentralPanel, Color32, ColorImage, Context, Frame, Image, Layout, Margin, Rect,
    RichText, TextureHandle, TextureOptions, UiBuilder, Vec2, load::SizedTexture, pos2, vec2,
};
use log::warn;
use mpris::PlayerFinder;
use std::{
    io::Read,
    time::{Duration, Instant},
};

const PANEL_MARGIN: f32 = 12.0;
const BLUR_SIGMA: f32 = 12.0;
const ART_RETRY_DELAY_SECS: u64 = 10;

fn main() -> eframe::Result<()> {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Cantus")
            .with_app_id("cantus")
            .with_inner_size([320.0, 120.0])
            .with_active(false)
            .with_window_level(egui::WindowLevel::AlwaysOnTop)
            .with_decorations(false),
        ..Default::default()
    };

    eframe::run_native(
        "Cantus",
        native_options,
        Box::new(|_cc| Ok(Box::new(CantusApp::new()))),
    )
}

struct CantusApp {
    last_poll: Instant,
    track: Option<TrackInfo>,
    album_art: Option<AlbumArtTextures>,
    current_art_url: Option<String>,
    next_art_retry: Option<Instant>,
    texture_seq: u64,
}

impl CantusApp {
    fn new() -> Self {
        let mut app = Self {
            last_poll: Instant::now(),
            track: None,
            album_art: None,
            current_art_url: None,
            next_art_retry: None,
            texture_seq: 0,
        };

        app.refresh_track();
        app
    }

    fn refresh_track(&mut self) {
        let art_was = self
            .track
            .as_ref()
            .and_then(|track| track.album_art_url.as_deref());

        let track = PlayerFinder::new()
            .ok()
            .and_then(|finder| finder.find_active().ok())
            .and_then(|player| player.get_metadata().ok())
            .map(|metadata| TrackInfo::from_metadata(&metadata));

        let track_missing = track.is_none();
        let art_changed = art_was
            != track
                .as_ref()
                .and_then(|track| track.album_art_url.as_deref());

        self.track = track;

        if track_missing || art_changed {
            self.album_art = None;
            self.current_art_url = None;
            self.next_art_retry = None;
        }
    }

    fn ensure_album_art(&mut self, ctx: &Context) {
        let Some(url) = self
            .track
            .as_ref()
            .and_then(|track| track.album_art_url.as_ref())
            .filter(|value| !value.is_empty())
            .cloned()
        else {
            self.album_art = None;
            self.current_art_url = None;
            self.next_art_retry = None;
            return;
        };

        let url_str = url.as_str();
        let same_url = self.current_art_url.as_deref() == Some(url_str);

        if same_url && self.album_art.is_some() {
            return;
        }

        if same_url
            && self
                .next_art_retry
                .is_some_and(|instant| Instant::now() < instant)
        {
            return;
        }

        if !same_url {
            self.album_art = None;
            self.next_art_retry = None;
        }

        match self.load_album_art(ctx, url_str) {
            Ok(textures) => {
                self.album_art = Some(textures);
                self.next_art_retry = None;
            }
            Err(err) => {
                warn!("Failed to load album art from {url}: {err}");
                self.album_art = None;
                self.next_art_retry =
                    Some(Instant::now() + Duration::from_secs(ART_RETRY_DELAY_SECS));
            }
        }

        self.current_art_url = Some(url);
    }

    fn load_album_art(&mut self, ctx: &Context, url: &str) -> Result<AlbumArtTextures, String> {
        let mut bytes = Vec::new();
        let response = ureq::get(url)
            .call()
            .map_err(|err| format!("HTTP request failed: {err}"))?;

        let status = response.status();
        if !status.is_success() {
            return Err(format!("HTTP request returned status {}", status.as_u16()));
        }

        response
            .into_body()
            .into_reader()
            .read_to_end(&mut bytes)
            .map_err(|err| format!("failed to read HTTP body: {err}"))?;

        let rgba = image::load_from_memory(&bytes)
            .map_err(|err| format!("failed to decode album art image: {err}"))?
            .to_rgba8();
        let blurred = image::imageops::blur(&rgba, BLUR_SIGMA);

        let next_name = |seq: &mut u64| {
            *seq = seq.wrapping_add(1);
            format!("album_art_{seq:010}")
        };

        let original = ctx.load_texture(
            next_name(&mut self.texture_seq),
            ColorImage::from_rgba_unmultiplied(
                [rgba.width() as usize, rgba.height() as usize],
                rgba.as_raw(),
            ),
            TextureOptions::LINEAR,
        );

        let blurred = ctx.load_texture(
            next_name(&mut self.texture_seq),
            ColorImage::from_rgba_unmultiplied(
                [blurred.width() as usize, blurred.height() as usize],
                blurred.as_raw(),
            ),
            TextureOptions::LINEAR,
        );

        Ok(AlbumArtTextures { original, blurred })
    }
}

impl eframe::App for CantusApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if self.last_poll.elapsed() >= Duration::from_millis(500) {
            self.refresh_track();
            self.last_poll = Instant::now();
        }

        self.ensure_album_art(ctx);

        CentralPanel::default()
            .frame(Frame {
                inner_margin: Margin::same(0),
                ..Default::default()
            })
            .show(ctx, |ui| {
                let full_rect = ui.max_rect();

                // Render the blurred album art over the entire background
                if let Some(album_art) = &self.album_art {
                    let painter = ui.painter_at(full_rect);
                    painter.image(
                        album_art.blurred.id(),
                        full_rect,
                        Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                        Color32::WHITE,
                    );
                    painter.rect_filled(
                        full_rect,
                        0.0,
                        Color32::from_rgba_unmultiplied(10, 10, 10, 170),
                    );
                }

                let content_rect = full_rect.shrink2(Vec2::splat(PANEL_MARGIN));
                let mut content_ui = ui.new_child(
                    UiBuilder::new()
                        .max_rect(content_rect)
                        .layout(Layout::left_to_right(Align::Center)),
                );

                if self.album_art.is_some() {
                    content_ui.visuals_mut().override_text_color =
                        Some(Color32::from_rgb(240, 240, 240));
                }

                let art_size = Vec2::splat(content_rect.height().max(0.0));
                if let Some(album_art) = &self.album_art {
                    let texture_size = album_art.original.size_vec2();
                    let uv_rect = if texture_size.x > texture_size.y {
                        let crop = (texture_size.x - texture_size.y) / (2.0 * texture_size.x);
                        Rect::from_min_max(pos2(crop, 0.0), pos2(1.0 - crop, 1.0))
                    } else if texture_size.y > texture_size.x {
                        let crop = (texture_size.y - texture_size.x) / (2.0 * texture_size.y);
                        Rect::from_min_max(pos2(0.0, crop), pos2(1.0, 1.0 - crop))
                    } else {
                        Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0))
                    };

                    content_ui.add(
                        Image::from_texture(SizedTexture::from_handle(&album_art.original))
                            .uv(uv_rect)
                            .fit_to_exact_size(art_size)
                            .corner_radius(8.0),
                    );
                }

                content_ui.add_space(10.0);
                let text_width = (content_rect.width() - art_size.x - 10.0).max(0.0);
                content_ui.allocate_ui_with_layout(
                    vec2(text_width, art_size.y),
                    Layout::top_down(Align::LEFT).with_main_align(Align::Center),
                    |ui| {
                        if let Some(track) = &self.track {
                            ui.label(RichText::new(&track.title).strong().size(20.0));
                            ui.label(&track.artist);
                            if let Some(album) = &track.album
                                && !album.is_empty()
                            {
                                ui.label(album);
                            }
                        } else {
                            ui.label("Nothing playing right now.");
                        }
                    },
                );
            });

        ctx.request_repaint_after(Duration::from_millis(300));
    }
}

struct TrackInfo {
    title: String,
    artist: String,
    album: Option<String>,
    album_art_url: Option<String>,
}

impl TrackInfo {
    fn from_metadata(metadata: &mpris::Metadata) -> Self {
        let title = metadata
            .title()
            .map_or_else(|| "Unknown Title".to_owned(), ToOwned::to_owned);

        let artist = metadata
            .artists()
            .filter(|artists| !artists.is_empty())
            .map_or_else(|| "Unknown Artist".to_owned(), |artists| artists.join(", "));

        let album = metadata.album_name().map(std::borrow::ToOwned::to_owned);
        let album_art_url = metadata
            .art_url()
            .map(std::borrow::ToOwned::to_owned)
            .filter(|url| !url.is_empty());

        Self {
            title,
            artist,
            album,
            album_art_url,
        }
    }
}

struct AlbumArtTextures {
    original: TextureHandle,
    blurred: TextureHandle,
}
