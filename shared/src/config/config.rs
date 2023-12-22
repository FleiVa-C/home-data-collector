use serde::{Deserialize, Serialize};
use std::{fs, io};
use toml;

#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    shellyv1: Option<ConfigTomlShellyV1>,
    shellyv2: Option<ConfigTomlShellyV2>,
    weather: Option<ConfigTomlWeather>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlShellyV1 {
    url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlShellyV2 {
    url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlWeather {
    url: Option<String>,
    api_key: Option<String>,
    station_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub shelly_v1_url: String,
    pub shelly_v2_url: String,
    pub weather_url: String,
    pub weather_api_key: String,
    pub weather_station_id: String,
}

impl Config {
    pub fn new() -> Self {
        let config_filepaths: [&str; 2] = ["./config.toml", "./Config.toml"];

        let mut content: String = String::new();

        for filepath in config_filepaths {
            let config: io::Result<String> = fs::read_to_string(filepath);

            if config.is_ok() {
                content = config.unwrap();
                break;
            }
        }

        let config_toml: ConfigToml = toml::from_str(&content).unwrap_or_else(|_| {
            println!("Failed to read Config.");
            ConfigToml {
                shellyv1: None,
                shellyv2: None,
                weather: None,
            }
        });

        let shelly_v1_url: String = match config_toml.shellyv1 {
            Some(shellyv1) => shellyv1.url.unwrap_or_else(|| {
                println!("Missing field shelly_v1_url in table shellyv1");
                "unknown".to_string()
            }),
            None => {
                println!("Missing table shellyv1");
                "unknown".to_string()
            }
        };

        let shelly_v2_url: String = match config_toml.shellyv2 {
            Some(shellyv2) => shellyv2.url.unwrap_or_else(|| {
                println!("Missing field shelly_v2_url in table shellyv2");
                "unknown".to_string()
            }),
            None => {
                println!("Missing table shellyv2");
                "unknown".to_string()
            }
        };

        let (weather_url, weather_api_key, weather_station_id): (String, String, String) =
            match config_toml.weather {
                Some(weather) => {
                    let url: String = weather.url.unwrap_or_else(|| {
                        println!("Missing field url in table weather");
                        "unknown".to_string()
                    });

                    let api_key: String = weather.api_key.unwrap_or_else(|| {
                        println!("Missing field api_key in table weather");
                        "unknown".to_string()
                    });

                    let station_id: String = weather.station_id.unwrap_or_else(|| {
                        println!("Missing field station_id in table weather");
                        "unknown".to_string()
                    });

                    (url, api_key, station_id)
                }

                None => {
                    println!("Missing table weather");
                    (
                        "unknown".to_string(),
                        "unknown".to_string(),
                        "unknown".to_string(),
                    )
                }
            };

        Config {
            shelly_v1_url,
            shelly_v2_url,
            weather_url,
            weather_api_key,
            weather_station_id,
        }
    }
}
