use serde::{de, de::Unexpected, Deserialize, Deserializer};

/// Helper to produce boolean from integer when deserializing.
/// Useful since many data send by torrent client via [`Announce`] uses integers rather than booleans.
pub fn bool_from_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
  D: Deserializer<'de>,
{
  match u8::deserialize(deserializer)? {
    0 => Ok(Some(false)),
    1 => Ok(Some(true)),
    other => Err(de::Error::invalid_value(
      Unexpected::Unsigned(other as u64),
      &"zero or one",
    )),
  }
}
