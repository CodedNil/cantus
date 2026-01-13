use fdsm::{generate::generate_msdf, shape::Shape, transform::Transform};
use image::ImageBuffer;
use nalgebra::Affine2;
use std::collections::HashMap;
use tracing::error;
use ttf_parser::{Face, GlyphId, Rect};

pub const ATLAS_MSDF_SCALE: f32 = 0.08;
pub const ATLAS_RANGE: f32 = 8.0;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextInstance {
    pub rect: [f32; 4],
    pub uv_rect: [f32; 4],
    pub color: [f32; 4],
}

pub struct GlyphInfo {
    pub uv_rect: [f32; 4],
    pub metrics: Rect,
}

pub struct MSDFAtlas {
    pub texture_data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub glyphs: HashMap<u32, GlyphInfo>,
}

impl MSDFAtlas {
    pub fn new(face: &Face, _size: u32) -> Self {
        let mut atlas = Self {
            texture_data: vec![0; (2048 * 2048 * 4) as usize],
            width: 2048,
            height: 2048,
            glyphs: HashMap::new(),
        };

        let (mut x, mut y, mut row_h) = (2, 2, 0);

        for c in (32u8..127)
            .map(|b| b as char)
            .chain((160u8..=255).map(|b| b as char))
            .chain("•".chars())
        {
            if let Some(gid) = face.glyph_index(c)
                && let Some((info, gh)) = atlas.create_glyph(face, gid, &mut x, &mut y, &mut row_h)
            {
                atlas.glyphs.insert(u32::from(gid.0), info);
                row_h = row_h.max(gh);
            }
        }
        atlas
    }

    fn create_glyph(
        &mut self,
        face: &Face,
        gid: GlyphId,
        x: &mut u32,
        y: &mut u32,
        row_h: &mut u32,
    ) -> Option<(GlyphInfo, u32)> {
        let shape = fdsm_ttf_parser::load_shape_from_face(face, gid)?;
        let bbox = face.glyph_bounding_box(gid)?;

        let (range, scale) = (ATLAS_RANGE, ATLAS_MSDF_SCALE);
        let gw = ((f32::from(bbox.width()) * scale) + (range * 2.0) + 3.0) as u32;
        let gh = ((f32::from(bbox.height()) * scale) + (range * 2.0) + 3.0) as u32;

        if *x + gw + 2 > self.width {
            *x = 2;
            *y += *row_h + 2;
            *row_h = 0;
        }
        if *y + gh + 2 > self.height {
            error!("Atlas full! Could not fit glyph ID {gid:?}");
            return None;
        }

        let mut msdf_img = ImageBuffer::<image::Rgb<f32>, Vec<f32>>::new(gw, gh);
        let mut shape = Shape::edge_coloring_simple(shape, 1.0, 0);

        let tx = (f64::from(-bbox.x_min) * f64::from(scale)) + f64::from(range) + 1.5;
        let ty = (f64::from(-bbox.y_min) * f64::from(scale)) + f64::from(range) + 1.5;
        shape.transform(&Affine2::from_matrix_unchecked(nalgebra::Matrix3::new(
            f64::from(scale),
            0.0,
            tx,
            0.0,
            f64::from(scale),
            ty,
            0.0,
            0.0,
            1.0,
        )));

        generate_msdf(&shape.prepare(), f64::from(range), &mut msdf_img);

        for (gx, gy, pixel) in msdf_img.enumerate_pixels() {
            let idx = ((*y + gy) * self.width + (*x + gx)) as usize * 4;
            let p0 = (pixel[0] * 255.0) as u8;
            let p1 = (pixel[1] * 255.0) as u8;
            let p2 = (pixel[2] * 255.0) as u8;
            self.texture_data[idx..idx + 4].copy_from_slice(&[p0, p1, p2, 255]);
        }

        let info = GlyphInfo {
            uv_rect: [
                *x as f32 / self.width as f32,
                *y as f32 / self.height as f32,
                gw as f32 / self.width as f32,
                gh as f32 / self.height as f32,
            ],
            metrics: bbox,
        };

        *x += gw + 2;
        Some((info, gh))
    }
}
