use super::DeviceType;
use serde::{Deserialize, Serialize};

/// Device object
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Device {
    pub id: Option<String>,
    pub is_active: bool,
    pub is_private_session: bool,
    pub is_restricted: bool,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: DeviceType,
    pub volume_percent: Option<u32>,
}

/// Intermediate device payload object
#[derive(Deserialize)]
pub struct DevicePayload {
    pub devices: Vec<Device>,
}
