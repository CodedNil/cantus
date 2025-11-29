use serde::Deserialize;

/// Image object
#[derive(Deserialize)]
pub struct Image {
    pub url: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}
