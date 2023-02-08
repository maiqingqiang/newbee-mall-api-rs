use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub url: String,
}

lazy_static! {
    pub static ref DATABASE: Config = envy::prefixed("DATABASE_").from_env::<Config>().unwrap();
}
