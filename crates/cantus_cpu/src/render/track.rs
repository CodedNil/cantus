use super::{Timeline, approach};
use crate::{
    CantusApp, PANEL_START, TRACK_SPACING_MS,
    config::Config,
    interaction::{InteractionState, Rect, TrackAction},
    spotify::{CondensedPlaylist, Track, playlist_icons},
};
use cantus_shared::{
    GAP,
    track::{AudioFeatures, ICON_WIDTH, MAX_PILL_PLAYLIST_ICONS, TrackPill},
};
use std::ops::Range;

const DEFAULT_AUDIO_FEATURES: AudioFeatures = AudioFeatures {
    energy: 0.5,
    danceability: 0.5,
    acousticness: 0.3,
    tempo: 120.0,
    valence: 0.5,
    instrumentalness: 0.1,
    loudness: -10.0,
};
const DETAIL_FADE_DURATION: f32 = 0.2;
const PLAYLIST_EXPANSION_DURATION: f32 = 1.0 / 6.0;

pub fn layout(queue: &mut [Track], config: &Config, timeline: Timeline, current_ms: f32) {
    let end_ms = (config.timeline_future_minutes - config.timeline_past_minutes) * 60_000.0;
    let gap = TRACK_SPACING_MS * timeline.px_per_ms;
    let width_trim = (GAP - gap).max(0.0);
    let end_x = config.history_width
        + config.timeline_future_minutes * 60_000.0 * timeline.px_per_ms
        + width_trim;
    let mut compact_count = 0;
    let mut transition = 0.0;
    let mut queue_offset = 0.0;

    for track in &mut *queue {
        track.runtime.width = 0.0;
        let start_ms = current_ms + queue_offset;
        queue_offset += track.queue_span_ms();
        if start_ms > end_ms {
            continue;
        }
        track.runtime.start_ms = start_ms;
        let (natural_start, natural_end) =
            track.natural_x_range(timeline.playhead_x, timeline.px_per_ms);
        if natural_end >= config.history_width + config.height {
            track.runtime.start_x = natural_start.max(config.history_width);
            track.runtime.width = (natural_end.min(end_x) - track.runtime.start_x - width_trim).max(0.0);
        } else if natural_end >= config.history_width {
            transition = (config.history_width + config.height - natural_end) / config.height;
            track.runtime.start_x = natural_end - config.height;
            track.runtime.width = config.height;
        } else {
            compact_count += 1;
        }
    }

    let stride = config.height * 0.55;
    for (index, track) in queue[..compact_count].iter_mut().enumerate() {
        let slot = compact_count - index - 1;
        let right = config.history_width - gap - (slot as f32 + transition) * stride;
        track.runtime.start_x = right - config.height;
        track.runtime.width = config.height;
    }
}

impl CantusApp {
    pub fn draw_track(
        &mut self,
        track: &mut Track,
        dt: f32,
        playlists: &[CondensedPlaylist],
        timeline: Timeline,
        ui: &mut InteractionState,
    ) -> (TrackPill, Range<u32>, bool, Option<TrackAction>) {
        let glyph_start = self.render.gpu.as_ref().unwrap().text_renderer.glyphs.len() as u32;
        let show_details = track.runtime.width > self.config.height;
        approach(
            &mut track.runtime.detail_alpha,
            f32::from(show_details),
            dt / DETAIL_FADE_DURATION,
        );
        let detail_alpha = track.runtime.detail_alpha;
        let playlist_expansion = track.runtime.playlist_expansion_curve();
        let mut pill = TrackPill {
            x: track.runtime.start_x,
            width: track.runtime.width.max(self.config.height),
            colors: track.runtime.art.palette(),
            visibility: detail_alpha.max(f32::from(track.runtime.start_ms <= 0.0)),
            image_index: self.get_image_index(&track.runtime.art),
            rating: -1,
            audio_features: track.audio_features.unwrap_or(DEFAULT_AUDIO_FEATURES),
            playlist_images: [-1; MAX_PILL_PLAYLIST_ICONS],
            ..Default::default()
        };

        if show_details && detail_alpha > 0.0 {
            let scale = self.render.scale;
            let gpu = self.render.gpu();
            gpu.text_renderer.render(&gpu.queue, track, detail_alpha, scale);
        }
        if show_details && let Some(track_id) = track.id {
            let icons = playlist_icons(track_id, playlists, true).chain(
                playlist_icons(track_id, playlists, false)
                    .filter(|_| track.runtime.playlist_expansion > 0.0),
            );
            for (slot, playlist) in pill.playlist_images.iter_mut().zip(icons) {
                *slot = self.get_image_index(&playlist.art);
                let primary = playlist.tracks.contains(&track_id);
                pill.primary_playlist_count += u32::from(primary);
                pill.secondary_playlist_count += u32::from(!primary);
            }
            pill.rating = if self.config.ratings_enabled {
                playlists
                    .iter()
                    .find_map(|playlist| {
                        playlist
                            .rating_index
                            .filter(|_| playlist.tracks.contains(&track_id))
                    })
                    .map_or(0, |rating| i32::from(rating) + 1)
            } else {
                -1
            };
        }
        track.runtime.primary_playlist_count = pill.primary_playlist_count as u8;
        let primary_icons = pill.star_count() + pill.primary_playlist_count as f32;
        approach(
            &mut track.runtime.primary_icon_alpha,
            f32::from(primary_icons > 0.0 && track.runtime.primary_icons_fit),
            dt / DETAIL_FADE_DURATION,
        );
        pill.primary_alpha = track
            .runtime
            .primary_icon_alpha
            .max(playlist_expansion * f32::from(primary_icons > 0.0));
        pill.secondary_expansion = playlist_expansion;

        let mut hovered = false;
        let mut action = None;
        if let Some(track_id) = track.id {
            let (primary, secondary) = pill.icon_rows(PANEL_START, self.config.height);
            let stars = pill.star_count() as usize;
            for (row, visible, primary_row) in [
                (primary, pill.primary_alpha > 0.0, true),
                (secondary, secondary.count > 0.0, false),
            ] {
                if !visible {
                    continue;
                }
                let response =
                    ui.surface(Rect::from_center(row.center, row.half_size(ICON_WIDTH * 0.5)));
                hovered |= response.hovered;
                if let Some((index, right_half)) = row.hit(ui.pointer()) {
                    if primary_row && response.hovered && index < stars {
                        pill.rating = index as i32 * 2 + 1 + i32::from(right_half);
                    }
                    if response.clicked {
                        action = if primary_row && index < stars {
                            Some(TrackAction::Rate(
                                track_id,
                                index as u8 * 2 + u8::from(right_half),
                            ))
                        } else {
                            playlist_icons(track_id, playlists, primary_row)
                                .nth(index - stars * usize::from(primary_row))
                                .map(|playlist| TrackAction::TogglePlaylist(track_id, playlist.id))
                        };
                    }
                }
            }
        }

        let body = ui.surface(Rect::pill(pill.x, pill.width, self.config.height));
        hovered |= body.hovered;
        if body.pressed {
            ui.enable_drag();
        }
        if body.clicked
            && let Some(track_id) = track.id
        {
            let (start, end) = track.natural_x_range(timeline.playhead_x, timeline.px_per_ms);
            let position = if ui.pointer().x < self.config.history_width + 40.0 {
                0.0
            } else {
                (ui.pointer().x - start) / (end - start)
            };
            action = Some(TrackAction::Seek(track_id, position));
        }
        approach(
            &mut track.runtime.playlist_expansion,
            f32::from(hovered && show_details && detail_alpha >= 1.0),
            dt.min(0.1) / PLAYLIST_EXPANSION_DURATION,
        );
        let glyph_end = self.render.gpu.as_ref().unwrap().text_renderer.glyphs.len() as u32;
        (pill, glyph_start..glyph_end, hovered, action)
    }
}
