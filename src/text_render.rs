use fdsm::{generate::generate_msdf, shape::Shape, transform::Transform};
use fdsm_ttf_parser::load_shape_from_face;
use image::ImageBuffer;
use nalgebra::{Affine2, Matrix3};
use std::collections::HashMap;
use tracing::error;
use ttf_parser::{Face, GlyphId, Rect};

pub const ATLAS_SCALE: f32 = 0.08;

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
        let bbox = face.glyph_bounding_box(gid)?;

        let gw = (f32::from(bbox.width()) * ATLAS_SCALE).ceil() as u32 + 2;
        let gh = (f32::from(bbox.height()) * ATLAS_SCALE).ceil() as u32 + 2;

        if *x + gw + 2 > self.width {
            *x = 2;
            *y += *row_h + 2;
            *row_h = 0;
        }
        if *y + gh + 2 > self.height {
            error!("Atlas full! Could not fit glyph ID {gid:?}");
            return None;
        }

        let shape = load_shape_from_face(face, gid)?;
        let mut shape = Shape::edge_coloring_simple(shape, 1.0, 0);
        let mut msdf_img = ImageBuffer::<image::Rgb<f32>, Vec<f32>>::new(gw, gh);

        let tx = f64::from(-bbox.x_min) * f64::from(ATLAS_SCALE) + 1.0;
        let ty = f64::from(-bbox.y_min) * f64::from(ATLAS_SCALE) + 1.0;

        shape.transform(&Affine2::from_matrix_unchecked(Matrix3::new(
            f64::from(ATLAS_SCALE),
            0.0,
            tx,
            0.0,
            f64::from(ATLAS_SCALE),
            ty,
            0.0,
            0.0,
            1.0,
        )));

        generate_msdf(&shape.prepare(), 2.0, &mut msdf_img);

        for (gx, gy, pixel) in msdf_img.enumerate_pixels() {
            let idx = ((*y + gy) * self.width + (*x + gx)) as usize * 4;
            self.texture_data[idx..idx + 4].copy_from_slice(&[
                (pixel[0] * 255.0) as u8,
                (pixel[1] * 255.0) as u8,
                (pixel[2] * 255.0) as u8,
                255,
            ]);
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
