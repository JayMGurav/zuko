pub mod fuzzy_matcher;



pub fn parse_optional_json<T: serde::de::DeserializeOwned>(
    value: Option<String>,
) -> Result<Option<T>, serde_json::Error> {
    match value {
        Some(ref s) if !s.trim().is_empty() => serde_json::from_str::<T>(s).map(Some),
        _ => Ok(None),
    }
}