use serde::{self, de::Error, de::DeserializeOwned, Deserializer, Deserialize};
use serde_json;

    // This generic function can deserialize any type T from a JSON string
    // that is stored in the database.
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    // THIS IS THE FIX:
    // We use `DeserializeOwned` to tell Serde that our `T` (e.g., Topic)
    // does not borrow any data from the string `s` it is parsed from.
    // This resolves the lifetime conflict.
    T: DeserializeOwned,
{
    // Deserialize the field from the database row as an Option<String>
    let opt_s = Option::<String>::deserialize(deserializer)?;

    match opt_s {
        Some(s) => {
            // If the string from the DB is empty, treat it as no data.
            if s.is_empty() {
                return Ok(None);
            }
            // Now, parse the string `s` using serde_json.
            // Because T is `DeserializeOwned`, Rust knows the result is
            // self-contained and won't borrow from `s`, which is about
            // to be destroyed at the end of this function.
            serde_json::from_str(&s).map(Some).map_err(Error::custom)
        }
        // If the database field is NULL, it will be None, so we return Ok(None)
        None => Ok(None),
    }
}
