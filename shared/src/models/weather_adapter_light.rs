use super::weather_adapter::WeatherAdapter;
use serde::{Deserialize, Serialize};
use std::iter::Iterator;

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherAdapterLight {
    pub temp: String,
    pub dewpoint: String,
    pub windchill: String,
    pub windspeed: String,
    pub windgust: String,
    pub pressure: String,
    pub preciprate: String,
    pub preciptotal: String,
    pub solar_radiaton: String,
    pub uv: String,
    pub wind_dir: String,
    pub humidity: String,
}

impl From<WeatherAdapter> for WeatherAdapterLight {
    fn from(value: WeatherAdapter) -> WeatherAdapterLight {
        WeatherAdapterLight {
            temp: String::from(value.temp),
            dewpoint: String::from(value.dewpoint),
            windchill: String::from(value.windchill),
            windspeed: String::from(value.windspeed),
            windgust: String::from(value.windgust),
            pressure: String::from(value.pressure),
            preciprate: String::from(value.preciprate),
            preciptotal: String::from(value.preciptotal),
            solar_radiaton: String::from(value.solar_radiation),
            uv: String::from(value.uv),
            wind_dir: String::from(value.wind_dir),
            humidity: String::from(value.humidity),
        }
    }
}

impl WeatherAdapterLight {
    pub fn iter(&self) -> WeatherIterator<'_> {
        WeatherIterator {
            inner: self,
            index: 0,
        }
    }
}

pub struct WeatherIterator<'a> {
    inner: &'a WeatherAdapterLight,
    index: u8,
}

impl<'a> Iterator for WeatherIterator<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => &self.inner.temp,
            1 => &self.inner.dewpoint,
            2 => &self.inner.windchill,
            3 => &self.inner.windspeed,
            4 => &self.inner.windgust,
            5 => &self.inner.preciprate,
            6 => &self.inner.preciptotal,
            7 => &self.inner.solar_radiaton,
            8 => &self.inner.uv,
            9 => &self.inner.wind_dir,
            10 => &self.inner.humidity,
            _ => return None,
        };
        self.index += 1;
        Some(ret)
    }
}
