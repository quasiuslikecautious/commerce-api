use serde::{ de, Deserialize, Deserializer };
use std::{ fmt, str::FromStr };

#[derive(Clone, Debug, Deserialize)]
pub struct Pagination {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    offset: Option<i64>,
    limit: Option<i64>,
}

/// Serde deserialization decorator to map empty Strings to None,
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

impl Pagination {
    pub fn get_offset(&self) -> i64 {
        self.offset.unwrap_or(0)
    }

    pub fn get_limit(&self) -> i64 {
        self.limit.unwrap_or(10)
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            offset: Some(0),
            limit: Some(10),
        }
    }
}
