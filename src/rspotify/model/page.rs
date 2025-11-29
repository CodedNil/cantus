use serde::{Deserialize, de::DeserializeOwned};

/// Custom deserializer to handle `Vec<Option<T>>` and filter out `None` values
/// This is useful for deserializing lists that may contain null values that are not relevants
fn vec_without_nulls<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    T: serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    let v = Vec::<Option<T>>::deserialize(deserializer)?;
    Ok(v.into_iter().flatten().collect())
}

#[derive(Deserialize)]
pub struct Page<T: DeserializeOwned> {
    #[serde(deserialize_with = "vec_without_nulls")]
    pub items: Vec<T>,
    pub total: u32,
}
