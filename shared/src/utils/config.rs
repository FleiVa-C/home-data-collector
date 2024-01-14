use serde::de::DeserializeOwned;
use std::{fs, io};
use toml;

pub enum ConfigError{
    ReadError,
    ParseError
}

impl From<io::Error> for ConfigError{
    fn from(_value: io::Error) -> Self {
        Self::ReadError
    }
}

impl From<toml::de::Error> for ConfigError{
    fn from(_value: toml::de::Error) -> Self {
        Self::ParseError
    }
}

pub fn load_config<T:DeserializeOwned>(path: &str) -> Result<T, ConfigError> {
    let config_content: String = fs::read_to_string(path)?;
    Ok(toml::from_str::<T>(&config_content)?)
}

