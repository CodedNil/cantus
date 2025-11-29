//! Custom serialization methods used throughout the crate

pub mod duration_ms {
    use chrono::Duration;
    use serde::de;
    use std::convert::TryFrom;
    use std::fmt;

    /// Vistor to help deserialize duration represented as millisecond to
    /// `chrono::Duration`.
    pub struct DurationVisitor;
    impl de::Visitor<'_> for DurationVisitor {
        type Value = Duration;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a milliseconds represents chrono::Duration")
        }
        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Duration::try_milliseconds(v).ok_or_else(|| {
                E::invalid_value(
                    serde::de::Unexpected::Signed(v),
                    &"an invalid duration in milliseconds",
                )
            })
        }

        // JSON deserializer calls visit_u64 for non-negative intgers
        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            i64::try_from(v).map_or_else(
                |_| {
                    Err(E::custom(format!(
                        "Conversion error: u64 to i64 conversion failed for value {v}"
                    )))
                },
                |val| {
                    Duration::try_milliseconds(val).ok_or_else(|| {
                        E::invalid_value(
                            serde::de::Unexpected::Signed(val),
                            &"a valid duration in
        milliseconds",
                        )
                    })
                },
            )
        }
    }

    /// Deserialize `chrono::Duration` from milliseconds (represented as i64)
    pub fn deserialize<'de, D>(d: D) -> Result<Duration, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_i64(DurationVisitor)
    }
}

pub mod option_duration_ms {
    use chrono::Duration;
    use serde::de;
    use std::fmt;

    use super::duration_ms;

    /// Vistor to help deserialize duration represented as milliseconds to
    /// `Option<chrono::Duration>`
    struct OptionDurationVisitor;

    impl<'de> de::Visitor<'de> for OptionDurationVisitor {
        type Value = Option<Duration>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(
                formatter,
                "a optional milliseconds represents chrono::Duration"
            )
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: de::Deserializer<'de>,
        {
            Ok(Some(
                deserializer.deserialize_i64(duration_ms::DurationVisitor)?,
            ))
        }
    }

    /// Deserialize `Option<chrono::Duration>` from milliseconds
    /// (represented as i64)
    pub fn deserialize<'de, D>(d: D) -> Result<Option<Duration>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        d.deserialize_option(OptionDurationVisitor)
    }
}

pub mod duration_second {
    use chrono::Duration;
    use serde::{Deserialize, Serializer, de};

    /// Deserialize `chrono::Duration` from seconds (represented as u64)
    pub fn deserialize<'de, D>(d: D) -> Result<Duration, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let duration: i64 = Deserialize::deserialize(d)?;
        Duration::try_seconds(duration).ok_or_else(|| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(duration),
                &"an invalid duration in seconds",
            )
        })
    }

    /// Serialize `chrono::Duration` to seconds (represented as u64)
    pub fn serialize<S>(x: &Duration, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_i64(x.num_seconds())
    }
}

pub mod space_separated_scopes {
    use serde::{Deserialize, Serializer, de};
    use std::collections::HashSet;

    pub fn deserialize<'de, D>(d: D) -> Result<HashSet<String>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let scopes: String = Deserialize::deserialize(d)?;
        Ok(scopes.split_whitespace().map(ToOwned::to_owned).collect())
    }

    pub fn serialize<S>(scopes: &HashSet<String>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let scopes = scopes.clone().into_iter().collect::<Vec<_>>().join(" ");
        s.serialize_str(&scopes)
    }
}

pub mod tracks_total {
    use serde::{Deserialize, Deserializer};

    #[derive(Deserialize)]
    struct TracksRef {
        total: u32,
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let tracks_ref = TracksRef::deserialize(deserializer)?;
        Ok(tracks_ref.total)
    }
}
