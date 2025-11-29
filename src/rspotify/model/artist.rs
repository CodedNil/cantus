use super::image::Image;
use arrayvec::ArrayString;
use serde::Deserialize;

pub type ArtistId = ArrayString<22>;

#[derive(Deserialize)]
pub struct Artist {
    pub id: ArtistId,
    pub name: String,
    #[serde(default)]
    pub images: Vec<Image>,
}

#[derive(Deserialize)]
pub struct Artists {
    pub artists: Vec<Artist>,
}
