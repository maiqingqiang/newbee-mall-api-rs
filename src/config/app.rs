use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub host: String,
    pub port: u16,
}

lazy_static! {
    pub static ref APP: Config = envy::prefixed("APP_").from_env::<Config>().unwrap();
}
