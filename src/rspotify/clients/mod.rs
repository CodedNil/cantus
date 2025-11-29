use crate::rspotify::ClientResult;
use serde::Deserialize;
use std::fmt::Write as _;

/// Converts a JSON response from Spotify into its model.
pub(crate) fn convert_result<'a, T: Deserialize<'a>>(input: &'a str) -> ClientResult<T> {
    serde_json::from_str::<T>(input).map_err(Into::into)
}

/// Append device ID to an API path.
pub(crate) fn append_device_id(path: &str, device_id: Option<&str>) -> String {
    let mut new_path = path.to_string();
    if let Some(device_id) = device_id {
        if path.contains('?') {
            let _ = write!(new_path, "&device_id={device_id}");
        } else {
            let _ = write!(new_path, "?device_id={device_id}");
        }
    }
    new_path
}
