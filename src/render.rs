use crate::spotify::{IMAGES_CACHE, PLAYBACK_STATE};
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

/// Build the scene for rendering.
pub fn create_scene(
    scene: &mut Scene,
    font_context: &mut FontContext,
    layout_context: &mut LayoutContext<()>,
    width: f64,
    height: f64,
    scale_factor: f64,
    background_image: Option<&ImageData>,
) {
    let scaled_panel_margin = PANEL_MARGIN * scale_factor;

    // Get current playback state
    let playback_state = PLAYBACK_STATE.lock().clone();
    let Some(song) = &playback_state.currently_playing else {
        return;
    };

    let background_rect = RoundedRect::new(0.0, 0.0, width, height, 14.0 * scale_factor);

    if let Some(image) = background_image {
        let brush = ImageBrush::new(image.clone());
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            &brush,
            None,
            &background_rect,
        );
    } else {
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            Color::new([0.9, 0.5, 0.6, 1.0]),
            None,
            &background_rect,
        );
    }

    // Draw the album art
    if let Some(image) = IMAGES_CACHE.get(&song.image.url) {
        scene.fill(
            Fill::NonZero,
            Affine::IDENTITY,
            &ImageBrush::new(image.original.clone()),
            None,
            &RoundedRect::new(
                scaled_panel_margin,
                scaled_panel_margin,
                height - scaled_panel_margin,
                height - scaled_panel_margin,
                10.0 * scale_factor,
            ),
        );
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
        scaled_panel_margin + (height - 2.0 * scaled_panel_margin) + (10.0 * scale_factor),
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
}
