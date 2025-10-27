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
    kurbo::{Affine, Rect, RoundedRect},
    peniko::{Color, Fill, ImageBrush},
};

/// Spacing between tracks in ms
const TRACK_SPACING_MS: f64 = 4000.0;
/// How many ms to show in the timeline
const TIMELINE_DURATION_MS: f64 = 12.0 * 60.0 * 1000.0;
/// Starting position of the timeline in ms, if negative then it shows the history too
const TIMELINE_START_MS: f64 = -40.0 * 1000.0;

const ROUNDING_RADIUS: f64 = 10.0;

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
        let mut track_spacing = 0.0;

        // Iterate over the currently playing track followed by the queued tracks.
        for track in std::iter::once(song).chain(playback_state.queue.iter()) {
            let track_start_ms_spaced = track_start_ms + track_spacing;
            if track_start_ms_spaced >= timeline_end_ms {
                break;
            }

            let track_end_ms = track_start_ms_spaced + track.milliseconds as f64;

            let visible_start_ms = track_start_ms_spaced.max(TIMELINE_START_MS);
            let visible_end_ms = track_end_ms.min(timeline_end_ms);

            let pos_x = (visible_start_ms - TIMELINE_START_MS) * px_per_ms;
            let width = (visible_end_ms - visible_start_ms) * px_per_ms;

            // If it starts before the timeline, then get the starting width to render differently
            let dark_width = if track_start_ms_spaced < 0.0 {
                track_start_ms_spaced.max(TIMELINE_START_MS) * -px_per_ms
            } else {
                0.0
            };

            // Draw the track, trimming to the visible window if it spills off either side.
            self.draw_track(
                id,
                track,
                pos_x,
                width,
                dark_width,
                total_height,
                track_start_ms,
            );

            track_start_ms += track.milliseconds as f64;
            track_spacing += TRACK_SPACING_MS;
        }

        // Purge the stale background cache entries.
        self.shader_backgrounds[id]
            .as_mut()
            .unwrap()
            .purge_stale(self.renderers[id].as_mut().unwrap(), self.frame_index);
    }

    fn draw_track(
        &mut self,
        id: usize,
        track: &Track,
        pos_x: f64,
        width: f64,
        dark_width: f64,
        height: f64,
        track_start_ms: f64,
    ) {
        let rounding_radius = ROUNDING_RADIUS * self.scale_factor;

        // For the part rendered behind the timeline
        let dark_reduction = 5.0;
        let dark_height = height - dark_reduction;

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
            if let Some(image) = background_image {
                let brush = ImageBrush::new(image.clone());

                // If theres any dark width, draw a thinner rectangle behind
                if dark_width > 0.0 {
                    let transform = Affine::translate((pos_x, 0.0));
                    let rect =
                        RoundedRect::new(0.0, dark_reduction, width, dark_height, rounding_radius);
                    self.scene
                        .fill(Fill::NonZero, transform, &brush, None, &rect);
                    // Darken it with a layer above
                    self.scene.fill(
                        Fill::NonZero,
                        transform,
                        Color::from_rgba8(0, 0, 0, 100),
                        None,
                        &rect,
                    );
                }

                let image_width = f64::from(image.width);
                let image_height = f64::from(image.height);
                let width = width - dark_width;
                self.scene.push_clip_layer(
                    Affine::translate((pos_x + dark_width, 0.0)),
                    &RoundedRect::new(0.0, 0.0, width, height, rounding_radius),
                );
                self.scene.fill(
                    Fill::NonZero,
                    Affine::translate((pos_x + dark_width, image_height * -0.5))
                        * Affine::scale_non_uniform(width / image_width, width / image_height),
                    &brush,
                    None,
                    &Rect::new(0.0, 0.0, image_width, image_height),
                );
                self.scene.pop_layer();
            }
        }

        // Draw the album art
        if let Some(image) = IMAGES_CACHE.get(&track.image.url) {
            let brush = ImageBrush::new(image.original.clone());
            let image_height = f64::from(image.original.height);

            // When the album art is clipping near the end, shrink it to move it onto the dark side
            let dark_offset = if width - dark_width < height {
                width - dark_width - height
            } else {
                0.0
            };
            let dark_transition = (dark_offset / height).abs().clamp(0.0, 1.0);
            let dark_reduction = dark_reduction * dark_transition;
            let dark_height = height - dark_reduction;

            // Render the primary album art
            let transform = Affine::translate((dark_offset + dark_width + pos_x, 0.0));
            let rect = RoundedRect::new(
                0.0,
                dark_reduction,
                dark_height,
                dark_height,
                rounding_radius,
            );
            self.scene.push_clip_layer(transform, &rect);
            self.scene.fill(
                Fill::NonZero,
                transform * Affine::scale(dark_height / image_height),
                &brush,
                None,
                &Rect::new(0.0, 0.0, image_height, image_height),
            );

            // Darken it with a layer above
            if dark_transition > 0.0 {
                self.scene.fill(
                    Fill::NonZero,
                    transform,
                    Color::from_rgba8(0, 0, 0, (100.0 * dark_transition).round() as u8),
                    None,
                    &rect,
                );
            }

            // Release clipping mask
            self.scene.pop_layer();
        }

        // Draw out text
        // Clipping mask to the edge of the background rectangle, shrunk by a margin
        let margin = 2.0 * self.scale_factor;
        self.scene.push_clip_layer(
            Affine::translate((pos_x, 0.0)),
            &RoundedRect::new(
                margin,
                margin,
                width - margin * 2.0,
                height - margin,
                rounding_radius,
            ),
        );

        // Render the songs title and artist (strip anything beyond a - or ( in the song title)
        let song_name = track.name[..track
            .name
            .find(" (")
            .or_else(|| track.name.find(" -"))
            .unwrap_or(track.name.len())]
            .trim();
        let text_start = dark_width + pos_x + height + (2.0 * self.scale_factor);
        self.draw_text(
            song_name,
            text_start,
            height * 0.35,
            14.0,
            FontWeight::EXTRA_BLACK,
        );

        // Draw the time until it starts
        let seconds_until_start = (track_start_ms / 1000.0).abs();
        let time_string = if seconds_until_start >= 60.0 {
            format!(
                "{}m {}s",
                (seconds_until_start / 60.0).floor(),
                (seconds_until_start % 60.0).floor()
            )
        } else {
            format!("{}s", seconds_until_start.round())
        };
        let time_width = self.draw_text(
            &time_string,
            text_start,
            height * 0.75,
            12.0,
            FontWeight::EXTRA_BLACK,
        );
        if let Some(artist_string) = track.artists.first().map(|artist| format!(" • {artist}")) {
            self.draw_text(
                &artist_string,
                text_start + ((time_width / 10.0).ceil() * 10.0),
                height * 0.75,
                12.0,
                FontWeight::EXTRA_BLACK,
            );
        }

        // Release clipping mask
        self.scene.pop_layer();
    }

    /// Renders out text with specified variables, returns how wide the text was.
    fn draw_text(
        &mut self,
        text: &str,
        pos_x: f64,
        pos_y: f64,
        font_size: f64,
        font_weight: FontWeight,
    ) -> f64 {
        let mut builder =
            self.layout_context
                .ranged_builder(&mut self.font_context, text, 1.0, false);
        builder.push_default(StyleProperty::FontStack(FontStack::Single(
            FontFamily::Named(Cow::Borrowed("epilogue")),
        )));
        builder.push_default(StyleProperty::FontSize(
            (font_size * self.scale_factor) as f32,
        ));
        builder.push_default(StyleProperty::FontWeight(font_weight));

        let mut layout: Layout<()> = builder.build(text);
        layout.break_all_lines(None);
        let text_transform = Affine::translate((
            pos_x + (10.0 * self.scale_factor),
            pos_y - (f64::from(layout.height()) * 0.5),
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

        f64::from(layout.width())
    }
}
