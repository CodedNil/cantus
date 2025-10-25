use crate::spotify::PLAYBACK_STATE;
use parley::{
    FontContext, FontFamily, FontStack, FontWeight, Layout, LayoutContext,
    layout::PositionedLayoutItem, style::StyleProperty,
};
use rspotify::model::PlayableItem;
use std::borrow::Cow;
use vello::{
    Glyph, Scene,
    kurbo::{Affine, RoundedRect},
    peniko::{Color, Fill},
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
) {
    let scaled_panel_margin = PANEL_MARGIN * scale_factor;

    // Draw a rectangle filling the screen
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        Color::new([0.9, 0.5, 0.6, 1.0]),
        None,
        &RoundedRect::new(0.0, 0.0, width, height, 14.0 * scale_factor),
    );

    // Draw the album art
    scene.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        Color::new([0.5, 0.0, 0.0, 1.0]),
        None,
        &RoundedRect::new(
            scaled_panel_margin,
            scaled_panel_margin,
            height - scaled_panel_margin,
            height - scaled_panel_margin,
            10.0 * scale_factor,
        ),
    );

    // Draw the text for song, album, and artist
    let playback_state = PLAYBACK_STATE.lock().clone();
    let Some(PlayableItem::Track(song)) = &playback_state.currently_playing else {
        return;
    };
    draw_text(
        scene,
        font_context,
        layout_context,
        &song.artists.first().map_or_else(
            || song.name.clone(),
            |artist| format!("{} • {}", song.name, artist.name),
        ),
        14.0 * scale_factor,
        Color::from_rgb8(240, 240, 240),
        FontWeight::EXTRA_BLACK,
        scaled_panel_margin + (height - 2.0 * scaled_panel_margin) + (10.0 * scale_factor),
        height * 0.5,
    );
}

/// Draw a single line of text into the scene.
fn draw_text(
    scene: &mut Scene,
    font_context: &mut FontContext,
    layout_context: &mut LayoutContext<()>,
    text: &str,
    font_size: f64,
    font_color: Color,
    font_weight: FontWeight,
    text_x: f64,
    text_y: f64,
) -> f64 {
    let mut builder = layout_context.ranged_builder(font_context, text, 1.0, false);
    builder.push_default(StyleProperty::FontStack(FontStack::Single(
        FontFamily::Named(Cow::Borrowed("epilogue")),
    )));
    builder.push_default(StyleProperty::FontSize(font_size as f32));
    builder.push_default(StyleProperty::FontWeight(font_weight));

    let mut layout: Layout<()> = builder.build(text);
    layout.break_all_lines(None);
    let text_transform = Affine::translate((text_x, text_y - (f64::from(layout.height()) / 2.0)));

    for item in layout.lines().flat_map(|line| line.items()) {
        let PositionedLayoutItem::GlyphRun(glyph_run) = item else {
            continue;
        };
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
            .brush(font_color)
            .draw(Fill::NonZero, glyphs);
    }

    f64::from(layout.width())
}
