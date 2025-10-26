use crate::spotify::{IMAGES_CACHE, PLAYBACK_STATE, Track};
use parley::{
    FontContext, FontFamily, FontStack, FontWeight, Layout, LayoutContext,
    layout::PositionedLayoutItem, style::StyleProperty,
};
use std::borrow::Cow;
use vello::{
    Glyph, Scene,
    kurbo::{Affine, RoundedRect},
    peniko::{Color, Fill, ImageBrush, ImageData},
};

const PANEL_MARGIN: f64 = 3.0;
const TIMELINE_DURATION_MS: f64 = 4.0 * 60.0 * 1000.0;
const TIMELINE_START_MS: f64 = -20.0 * 1000.0;

/// Build the scene for rendering.
pub fn create_scene(
    scene: &mut Scene,
    font_context: &mut FontContext,
    layout_context: &mut LayoutContext<()>,
    total_width: f64,
    total_height: f64,
    scale_factor: f64,
    background_image: Option<&ImageData>,
) {
    // Get current playback state
    let playback_state = PLAYBACK_STATE.lock().clone();

    let Some(song) = &playback_state.currently_playing else {
        return;
    };

    let timeline_end_ms = TIMELINE_START_MS + TIMELINE_DURATION_MS;
    let px_per_ms = total_width / TIMELINE_DURATION_MS;

    // Track positions are relative to "now" (0 ms), negative values are in the past.
    let mut track_start_ms = -(playback_state.progress as f64);

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
        draw_track(
            track,
            scene,
            font_context,
            layout_context,
            pos_x,
            width,
            total_height,
            scale_factor,
            background_image,
        );

        track_start_ms = track_end_ms;
    }
}

fn draw_track(
    song: &Track,
    scene: &mut Scene,
    font_context: &mut FontContext,
    layout_context: &mut LayoutContext<()>,
    pos_x: f64,
    width: f64,
    height: f64,
    scale_factor: f64,
    background_image: Option<&ImageData>,
) {
    // Draw the background using the shader
    let background_rect = RoundedRect::new(0.0, 0.0, width, height, 14.0 * scale_factor);
    let background_transform = Affine::translate((pos_x, 0.0));
    if let Some(image) = background_image {
        let brush = ImageBrush::new(image.clone());
        scene.fill(
            Fill::NonZero,
            background_transform,
            &brush,
            None,
            &background_rect,
        );
    } else {
        scene.fill(
            Fill::NonZero,
            background_transform,
            Color::new([0.9, 0.5, 0.6, 1.0]),
            None,
            &background_rect,
        );
    }

    // Clipping mask to the edge of the background rectangle
    let background_margin = 2.0 * scale_factor;
    let background_rect_shrunk = RoundedRect::new(
        background_margin,
        background_margin,
        width - background_margin,
        height - background_margin,
        12.0 * scale_factor,
    );
    scene.push_clip_layer(background_transform, &background_rect_shrunk);

    // Draw the album art
    if let Some(image) = IMAGES_CACHE.get(&song.image.url) {
        let panel_size = (height - 2.0 * PANEL_MARGIN).max(0.0);
        if panel_size > 0.0 {
            let image_data = &image.original;
            let img_w = f64::from(image_data.width.max(1));
            let img_h = f64::from(image_data.height.max(1));
            // Scale proportionally so the shorter edge fits; overflow is clipped to the square.
            let scale = panel_size / img_w.min(img_h);
            scene.fill(
                Fill::NonZero,
                Affine::translate((
                    pos_x + PANEL_MARGIN + (panel_size - img_w * scale) * 0.5,
                    PANEL_MARGIN + (panel_size - img_h * scale) * 0.5,
                )) * Affine::scale(scale),
                &ImageBrush::new(image_data.clone()),
                None,
                &RoundedRect::new(0.0, 0.0, img_w, img_h, 12.0 * scale_factor),
            );
        }
    }

    // Render the songs title and artist (strip anything beyond a - or ( in the song title)
    let song_name = song.name[..song
        .name
        .find(" (")
        .or_else(|| song.name.find(" -"))
        .unwrap_or(song.name.len())]
        .trim();
    let text = song.artists.first().map_or_else(
        || song_name.to_string(),
        |artist| format!("{song_name} • {artist}"),
    );

    let mut builder = layout_context.ranged_builder(font_context, &text, 1.0, false);
    builder.push_default(StyleProperty::FontStack(FontStack::Single(
        FontFamily::Named(Cow::Borrowed("epilogue")),
    )));
    builder.push_default(StyleProperty::FontSize((14.0 * scale_factor) as f32));
    builder.push_default(StyleProperty::FontWeight(FontWeight::EXTRA_BLACK));

    let mut layout: Layout<()> = builder.build(&text);
    layout.break_all_lines(None);
    let text_transform = Affine::translate((
        pos_x + PANEL_MARGIN + (height - 2.0 * PANEL_MARGIN) + (10.0 * scale_factor),
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
        scene
            .draw_glyphs(run.font())
            .font_size(run.font_size())
            .normalized_coords(run.normalized_coords())
            .transform(text_transform)
            .hint(true)
            .brush(Color::from_rgb8(240, 240, 240))
            .draw(Fill::NonZero, glyphs);
    }

    // Release clipping mask
    scene.pop_layer();
}
