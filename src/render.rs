use crate::{
    CantusLayer,
    background::WarpBackground,
    spotify::{IMAGES_CACHE, PLAYBACK_STATE, Track},
};
use parley::{
    FontFamily, FontStack, FontWeight, Layout, layout::PositionedLayoutItem, style::StyleProperty,
};
use std::borrow::Cow;
use vello::{
    Glyph,
    kurbo::{Affine, RoundedRect},
    peniko::{Color, Fill, ImageBrush},
};

const PANEL_MARGIN: f64 = 3.0;
const TIMELINE_DURATION_MS: f64 = 4.0 * 60.0 * 1000.0;
const TIMELINE_START_MS: f64 = -20.0 * 1000.0;

/// Build the scene for rendering.
impl CantusLayer {
    pub fn create_scene(&mut self, id: usize) {
        let surface = self.render_surface.as_ref().unwrap();
        let total_width = f64::from(surface.config.width);
        let total_height = f64::from(surface.config.height);

        // Get current playback state
        let playback_state = PLAYBACK_STATE.lock().clone();

        // Ensure the background provider exists
        if self.shader_backgrounds[id].is_none() {
            self.shader_backgrounds[id] =
                Some(WarpBackground::new(&self.render_context.devices[id].device));
        }

        let Some(song) = &playback_state.currently_playing else {
            return;
        };

        let timeline_end_ms = TIMELINE_START_MS + TIMELINE_DURATION_MS;
        let px_per_ms = total_width / TIMELINE_DURATION_MS;

        // Track positions are relative to "now" (0 ms), negative values are in the past.
        let lerped_progress = playback_state.progress
            + (u64::from(playback_state.playing)
                * playback_state.last_updated.elapsed().as_millis() as u64);
        let mut track_start_ms = -(lerped_progress as f64);

        // Iterate over the currently playing track followed by the queued tracks.
        for track in std::iter::once(song).chain(playback_state.queue.iter()) {
            if track_start_ms >= timeline_end_ms {
                break;
            }

            let track_end_ms = track_start_ms + track.milliseconds as f64;

            let visible_start_ms = track_start_ms.max(TIMELINE_START_MS);
            let visible_end_ms = track_end_ms.min(timeline_end_ms);
            if visible_end_ms <= visible_start_ms {
                track_start_ms = track_end_ms;
                continue;
            }

            let pos_x = (visible_start_ms - TIMELINE_START_MS) * px_per_ms;
            let width = (visible_end_ms - visible_start_ms) * px_per_ms;

            // Draw the track, trimming to the visible window if it spills off either side.
            self.draw_track(id, track, pos_x, width, total_height);

            track_start_ms = track_end_ms;
        }

        // Purge the stale background cache entries.
        self.shader_backgrounds[id]
            .as_mut()
            .unwrap()
            .purge_stale(self.renderers[id].as_mut().unwrap(), self.frame_index);
    }

    fn draw_track(&mut self, id: usize, track: &Track, pos_x: f64, width: f64, height: f64) {
        let background_transform = Affine::translate((pos_x, 0.0));
        if let Some(image) = IMAGES_CACHE.get(&track.image.url) {
            let surface = self.render_surface.as_ref().unwrap();
            let background_image = self.shader_backgrounds[id].as_mut().unwrap().render(
                &track.image.url,
                &self.render_context.devices[id],
                self.renderers[id].as_mut().unwrap(),
                surface.config.width,
                surface.config.height,
                &image.blurred,
                self.time_origin.elapsed().as_secs_f32(),
                self.frame_index,
            );

            // Draw the background using the shader
            let background_rect =
                RoundedRect::new(0.0, 0.0, width, height, 14.0 * self.scale_factor);
            if let Some(image) = background_image {
                let brush = ImageBrush::new(image);
                self.scene.fill(
                    Fill::NonZero,
                    background_transform,
                    &brush,
                    None,
                    &background_rect,
                );
            } else {
                self.scene.fill(
                    Fill::NonZero,
                    background_transform,
                    Color::new([0.9, 0.5, 0.6, 1.0]),
                    None,
                    &background_rect,
                );
            }
        }

        // Clipping mask to the edge of the background rectangle
        let background_margin = 2.0 * self.scale_factor;
        let background_rect_shrunk = RoundedRect::new(
            background_margin,
            background_margin,
            width - background_margin,
            height - background_margin,
            12.0 * self.scale_factor,
        );
        self.scene
            .push_clip_layer(background_transform, &background_rect_shrunk);

        // Draw the album art
        if let Some(image) = IMAGES_CACHE.get(&track.image.url) {
            let panel_size = (height - 2.0 * PANEL_MARGIN).max(0.0);
            if panel_size > 0.0 {
                let image_data = &image.original;
                let img_w = f64::from(image_data.width.max(1));
                let img_h = f64::from(image_data.height.max(1));
                // Scale proportionally so the shorter edge fits; overflow is clipped to the square.
                let scale = panel_size / img_w.min(img_h);
                self.scene.fill(
                    Fill::NonZero,
                    Affine::translate((
                        pos_x + PANEL_MARGIN + (panel_size - img_w * scale) * 0.5,
                        PANEL_MARGIN + (panel_size - img_h * scale) * 0.5,
                    )) * Affine::scale(scale),
                    &ImageBrush::new(image_data.clone()),
                    None,
                    &RoundedRect::new(0.0, 0.0, img_w, img_h, 12.0 * self.scale_factor),
                );
            }
        }

        // Render the songs title and artist (strip anything beyond a - or ( in the song title)
        let song_name = track.name[..track
            .name
            .find(" (")
            .or_else(|| track.name.find(" -"))
            .unwrap_or(track.name.len())]
            .trim();
        let text = track.artists.first().map_or_else(
            || song_name.to_string(),
            |artist| format!("{song_name} • {artist}"),
        );

        let mut builder =
            self.layout_context
                .ranged_builder(&mut self.font_context, &text, 1.0, false);
        builder.push_default(StyleProperty::FontStack(FontStack::Single(
            FontFamily::Named(Cow::Borrowed("epilogue")),
        )));
        builder.push_default(StyleProperty::FontSize((14.0 * self.scale_factor) as f32));
        builder.push_default(StyleProperty::FontWeight(FontWeight::EXTRA_BLACK));

        let mut layout: Layout<()> = builder.build(&text);
        layout.break_all_lines(None);
        let text_transform = Affine::translate((
            pos_x + PANEL_MARGIN + (height - 2.0 * PANEL_MARGIN) + (10.0 * self.scale_factor),
            (height * 0.5) - (f64::from(layout.height()) * 0.5),
        ));

        for glyph_run in layout
            .lines()
            .flat_map(|line| line.items())
            .filter_map(|item| {
                if let PositionedLayoutItem::GlyphRun(run) = item {
                    Some(run)
                } else {
                    None
                }
            })
        {
            let glyphs = glyph_run.positioned_glyphs().map(|g| Glyph {
                id: g.id,
                x: g.x,
                y: g.y,
            });
            let run = glyph_run.run();
            self.scene
                .draw_glyphs(run.font())
                .font_size(run.font_size())
                .normalized_coords(run.normalized_coords())
                .transform(text_transform)
                .hint(true)
                .brush(Color::from_rgb8(240, 240, 240))
                .draw(Fill::NonZero, glyphs);
        }

        // Release clipping mask
        self.scene.pop_layer();
    }
}
