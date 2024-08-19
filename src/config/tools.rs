use serde::{Deserialize, Deserializer};

pub fn deserialize_boolean<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: serde_json::Value = Deserialize::deserialize(deserializer)?;
    match s {
        serde_json::Value::Bool(b) => Ok(b),
        serde_json::Value::String(ref s) if s == "true" => Ok(true),
        serde_json::Value::String(ref s) if s == "false" => Ok(false),
        _ => Err(serde::de::Error::custom("expected a boolean or string 'true'/'false'")),
    }
}