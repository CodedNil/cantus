use crate::PANEL_START;
use crate::config::CONFIG;
use crate::render::TrackRender;
use wgpu::{Device, Queue, RenderPass};
use wgpu_text::{
    BrushBuilder, TextBrush,
    glyph_brush::{
        BuiltInLineBreaker, HorizontalAlign, Layout, OwnedSection, OwnedText, Section, Text,
        VerticalAlign, ab_glyph::FontArc, ab_glyph::PxScale,
    },
};

const FONT_SIZE: f32 = 17.0;
const FONT_SIZE_SMALL: f32 = 14.0;

pub struct TextRenderer {
    brush: TextBrush<FontArc>,
    sections: Vec<OwnedSection>,
}

impl TextRenderer {
    pub fn new(device: &Device, format: wgpu::TextureFormat) -> Self {
        let font = FontArc::try_from_slice(include_bytes!("../assets/NotoSans-Bold.ttf")).unwrap();
        Self {
            brush: BrushBuilder::using_font(font).build(device, 0, 0, format),
            sections: Vec::new(),
        }
    }

    pub fn render(&mut self, track_render: &TrackRender) {
        let track = track_render.track;
        let text_start_left = track_render.start_x + 12.0;
        let text_start_right = track_render.start_x + track_render.width - CONFIG.height - 8.0;
        let available_width = text_start_right - text_start_left;

        if available_width <= 0.0 {
            return;
        }

        let text_color = [0.94, 0.94, 0.94, (available_width / 100.0).min(1.0)];

        let mut queue_text =
            |text: String, pos: (f32, f32), size: f32, h_align: HorizontalAlign| {
                self.sections.push(OwnedSection {
                    screen_position: pos,
                    bounds: (available_width + 2.0, f32::INFINITY),
                    layout: Layout::SingleLine {
                        line_breaker: BuiltInLineBreaker::AnyCharLineBreaker,
                        h_align,
                        v_align: VerticalAlign::Center,
                    },
                    text: vec![OwnedText::new(text).with_scale(size).with_color(text_color)],
                });
            };

        let song_name = track
            .name
            .split(" -")
            .next()
            .unwrap_or(&track.name)
            .split('(')
            .next()
            .unwrap_or("")
            .trim();

        let top_y = PANEL_START + (CONFIG.height * 0.26).floor();
        let bottom_y = PANEL_START + (CONFIG.height * 0.57).floor();

        let measure_layout = Layout::SingleLine {
            line_breaker: BuiltInLineBreaker::AnyCharLineBreaker,
            h_align: HorizontalAlign::Left,
            v_align: VerticalAlign::Center,
        };

        let measured_width = self
            .brush
            .glyph_bounds(
                Section::default()
                    .add_text(Text::new(song_name).with_scale(FONT_SIZE))
                    .with_layout(measure_layout),
            )
            .map_or(0.0, |b| b.width());

        let width_ratio = available_width / measured_width;
        let (x, align, size) = if width_ratio <= 1.0 {
            (
                text_start_left,
                HorizontalAlign::Left,
                FONT_SIZE * width_ratio.max(0.8),
            )
        } else {
            (text_start_right, HorizontalAlign::Right, FONT_SIZE)
        };
        queue_text(song_name.to_owned(), (x, top_y), size, align);

        let time_text = if track_render.seconds_until_start >= 60.0 {
            format!(
                "{}m{}s",
                (track_render.seconds_until_start / 60.0).floor(),
                (track_render.seconds_until_start % 60.0).floor()
            )
        } else {
            format!("{}s", track_render.seconds_until_start.round())
        };

        let bottom_merged = format!("{time_text}\u{2004}•\u{2004}{}", track.artist.name);
        let measured_bottom_width = self
            .brush
            .glyph_bounds(
                Section::default()
                    .add_text(Text::new(&bottom_merged).with_scale(FONT_SIZE_SMALL))
                    .with_layout(measure_layout),
            )
            .map_or(0.0, |b| b.width());

        let bottom_ratio = available_width / measured_bottom_width;
        if bottom_ratio <= 1.0 || !track_render.is_current {
            let align = if bottom_ratio >= 1.0 {
                HorizontalAlign::Right
            } else {
                HorizontalAlign::Left
            };
            let x = if bottom_ratio >= 1.0 {
                text_start_right
            } else {
                text_start_left
            };
            queue_text(
                bottom_merged,
                (x, bottom_y),
                FONT_SIZE_SMALL * bottom_ratio.clamp(0.8, 1.0),
                align,
            );
        } else {
            queue_text(
                time_text,
                (text_start_left, bottom_y),
                FONT_SIZE_SMALL,
                HorizontalAlign::Left,
            );
            queue_text(
                track.artist.name.clone(),
                (text_start_right, bottom_y),
                FONT_SIZE_SMALL,
                HorizontalAlign::Right,
            );
        }
    }

    pub fn draw(
        &mut self,
        device: &Device,
        queue: &Queue,
        rpass: &mut RenderPass<'_>,
        width: u32,
        height: u32,
        scale: f32,
    ) {
        self.brush.update_matrix(
            [
                [2.0 / width as f32, 0.0, 0.0, 0.0],
                [0.0, -2.0 / height as f32, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [-1.0, 1.0, 0.0, 1.0],
            ],
            queue,
        );

        let sections = std::mem::take(&mut self.sections);
        let refs: Vec<Section> = sections
            .iter()
            .map(|s| Section {
                screen_position: (s.screen_position.0 * scale, s.screen_position.1 * scale),
                bounds: (s.bounds.0 * scale, s.bounds.1 * scale),
                layout: s.layout,
                text: s
                    .text
                    .iter()
                    .map(|t| Text {
                        text: &t.text,
                        scale: PxScale {
                            x: t.scale.x * scale,
                            y: t.scale.y * scale,
                        },
                        font_id: t.font_id,
                        extra: t.extra,
                    })
                    .collect(),
            })
            .collect();

        self.brush.queue(device, queue, refs).unwrap();
        self.brush.draw(rpass);
    }
}
