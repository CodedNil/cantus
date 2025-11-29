use serde::Deserialize;

/// Image object
#[derive(Deserialize)]
pub struct Image {
    pub url: String,
    pub height: Option<u32>,
    pub width: Option<u32>,
}
