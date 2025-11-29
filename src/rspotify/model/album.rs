use super::{artist::Artist, image::Image};
use arrayvec::ArrayString;
use serde::Deserialize;

pub type AlbumId = ArrayString<22>;

#[derive(Deserialize)]
pub struct Album {
    pub id: AlbumId,
    pub name: String,
    pub artists: Vec<Artist>,
    pub images: Vec<Image>,
}
