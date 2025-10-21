use eframe::egui;
use mpris::PlayerFinder;
use std::time::{Duration, Instant};

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
    player_finder: Option<PlayerFinder>,
    last_poll: Instant,
    track: Option<TrackInfo>,
    last_error: Option<String>,
}

impl CantusApp {
    fn new() -> Self {
        let mut app = Self {
            player_finder: PlayerFinder::new().ok(),
            last_poll: Instant::now(),
            track: None,
            last_error: None,
        };

        app.refresh_track();
        app
    }

    fn refresh_track(&mut self) {
        let Some(finder) = &self.player_finder else {
            self.track = None;
            self.last_error = Some("Could not connect to the MPRIS D-Bus service.".to_owned());
            return;
        };

        match finder.find_active() {
            Ok(player) => match player.get_metadata() {
                Ok(metadata) => {
                    self.track = Some(TrackInfo::from_metadata(&metadata));
                    self.last_error = None;
                }
                Err(err) => {
                    self.track = None;
                    self.last_error =
                        Some(format!("Failed to read metadata from active player: {err}"));
                }
            },
            Err(err) => {
                self.track = None;
                self.last_error = Some(format!("No active MPRIS player found: {err}"));
            }
        }
    }
}

impl eframe::App for CantusApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.last_poll.elapsed() >= Duration::from_millis(500) {
            self.refresh_track();
            self.last_poll = Instant::now();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Cantus");
                ui.add_space(6.0);

                if let Some(track) = &self.track {
                    ui.label(egui::RichText::new(&track.title).strong().size(18.0));
                    ui.label(&track.artist);
                    if let Some(album) = &track.album
                        && !album.is_empty()
                    {
                        ui.label(album);
                    }
                } else {
                    ui.label("Nothing playing right now.");
                }

                if let Some(error) = &self.last_error {
                    ui.add_space(8.0);
                    ui.colored_label(egui::Color32::from_rgb(200, 80, 80), error);
                }
            });
        });

        ctx.request_repaint_after(Duration::from_millis(300));
    }
}

struct TrackInfo {
    title: String,
    artist: String,
    album: Option<String>,
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

        Self {
            title,
            artist,
            album,
        }
    }
}
