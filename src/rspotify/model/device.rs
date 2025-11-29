use serde::Deserialize;

/// Device object
#[derive(Deserialize)]
pub struct Device {
    pub name: String,
    pub volume_percent: Option<u32>,
}
