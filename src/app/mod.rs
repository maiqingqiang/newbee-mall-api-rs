use serde::{Deserialize, Deserializer};
use std::str::FromStr;

pub mod admin;
pub mod mall;

fn de_empty_to_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    if s.is_empty() {
        return Ok(None);
    }

    Ok(Some(s.parse::<T>().unwrap()))
}

fn de_string_to_int<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + std::default::Default,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    if s.is_empty() {
        return Ok(T::default());
    }

    Ok(s.parse::<T>().unwrap())
}
