use super::{RenderState, Timeline, approach};
use crate::{
    PANEL_START, TRACK_SPACING_MS,
    config::Config,
    interaction::{InteractionState, Rect, TrackAction},
    spotify::{CondensedPlaylist, Track, playlist_icons},
};
use cantus_gpu::{
    GAP,
    track::{AudioFeatures, Data, ICON_SPACING, ICON_WIDTH, MAX_PILL_PLAYLIST_ICONS},
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

#[derive(Clone, Copy)]
pub struct TrackLayout {
    pub start_ms: f32,
    pub x: f32,
    pub width: f32,
}

impl TrackLayout {
    pub fn natural_x_range(self, track: &Track, timeline: Timeline) -> (f32, f32) {
        let start = timeline.playhead_x + self.start_ms * timeline.px_per_ms;
        (start, start + track.duration_ms as f32 * timeline.px_per_ms)
    }
}

pub fn layouts<'a>(
    queue: &'a mut [Track],
    config: &Config,
    timeline: Timeline,
    current_ms: f32,
) -> impl Iterator<Item = (usize, &'a mut Track, TrackLayout)> + 'a {
    let end_ms = (config.timeline_future_minutes - config.timeline_past_minutes) * 60_000.0;
    let gap = TRACK_SPACING_MS * timeline.px_per_ms;
    let width_trim = (GAP - gap).max(0.0);
    let end_x = config.history_width
        + config.timeline_future_minutes * 60_000.0 * timeline.px_per_ms
        + width_trim;
    let mut compact_slot = 0;
    let mut transition = 0.0;
    let mut queue_end_ms = current_ms + queue.iter().map(Track::queue_span_ms).sum::<f32>();
    let (history_width, panel_height) = (config.history_width, config.height);
    queue.iter_mut().enumerate().rev().map(move |(index, track)| {
        queue_end_ms -= track.queue_span_ms();
        let mut layout = TrackLayout {
            start_ms: queue_end_ms,
            x: 0.0,
            width: panel_height,
        };
        let (natural_start, natural_end) = layout.natural_x_range(track, timeline);
        if layout.start_ms > end_ms {
            layout.width = 0.0;
        } else if natural_end >= history_width + panel_height {
            layout.x = natural_start.max(history_width);
            layout.width = (natural_end.min(end_x) - layout.x - width_trim).max(0.0);
        } else if natural_end >= history_width {
            transition = (history_width + panel_height - natural_end) / panel_height;
            layout.x = natural_end - panel_height;
        } else {
            let right = history_width - gap - (compact_slot as f32 + transition) * panel_height * 0.55;
            compact_slot += 1;
            layout.x = right - panel_height;
        }
        (index, track, layout)
    })
}

impl RenderState {
    pub fn draw_track(
        &mut self,
        track: &mut Track,
        layout: &mut TrackLayout,
        dt: f32,
        config: &Config,
        playlists: &[CondensedPlaylist],
        timeline: Timeline,
        ui: &mut InteractionState,
    ) -> (Data, Range<u32>, bool, Option<TrackAction>) {
        let glyph_start = self.gpu.as_ref().unwrap().text_renderer.glyphs.len() as u32;
        let playlist_expansion = track.runtime.playlist_expansion_curve();
        if playlist_expansion > 0.0 {
            let target_width = self.gpu().text_renderer.track_width(track, layout.start_ms);
            let extra_width = (target_width - layout.width).max(0.0) * playlist_expansion;
            layout.x -= extra_width * 0.5;
            layout.width += extra_width;
        }
        let show_details = layout.width > config.height;
        approach(
            &mut track.runtime.detail_alpha,
            f32::from(show_details),
            dt / DETAIL_FADE_DURATION,
        );
        let detail_alpha = track.runtime.detail_alpha;
        let mut pill = Data {
            x: layout.x,
            width: layout.width.max(config.height),
            colors: track.runtime.art.palette(),
            visibility: detail_alpha.max(f32::from(layout.start_ms <= 0.0)),
            image_index: self.get_image_index(&track.runtime.art),
            rating: -1,
            audio_features: track.audio_features.unwrap_or(DEFAULT_AUDIO_FEATURES),
            playlist_images: [-1; MAX_PILL_PLAYLIST_ICONS],
            ..Default::default()
        };

        if show_details && detail_alpha > 0.0 {
            let scale = self.scale;
            let gpu = self.gpu();
            gpu.text_renderer.render(&gpu.queue, track, *layout, scale);
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
            pill.rating = if config.ratings_enabled {
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
        let primary_icons = pill.star_count() + pill.primary_playlist_count as f32;
        approach(
            &mut track.runtime.primary_icon_alpha,
            f32::from(primary_icons > 0.0 && layout.width >= ICON_SPACING * 1.05 * primary_icons),
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
            let (primary, secondary) = pill.icon_rows(PANEL_START, config.height);
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
                if let Some((index, right_half)) = row.hit(ui.pointer) {
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

        let body = ui.surface(Rect::pill(pill.x, pill.width, config.height));
        hovered |= body.hovered;
        if body.pressed {
            ui.drag_enabled = true;
        }
        if body.clicked
            && let Some(track_id) = track.id
        {
            let (start, end) = layout.natural_x_range(track, timeline);
            let position = if ui.pointer.x < config.history_width + 40.0 {
                0.0
            } else {
                (ui.pointer.x - start) / (end - start)
            };
            action = Some(TrackAction::Seek(track_id, position));
        }
        approach(
            &mut track.runtime.playlist_expansion,
            f32::from(hovered && show_details && detail_alpha >= 1.0),
            dt.min(0.1) / PLAYLIST_EXPANSION_DURATION,
        );
        let glyph_end = self.gpu.as_ref().unwrap().text_renderer.glyphs.len() as u32;
        (pill, glyph_start..glyph_end, hovered, action)
    }
}
