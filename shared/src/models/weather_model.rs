use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use std::iter::zip;
use std::time::SystemTime;

use crate::models;
use models::ingestion_container::*;
use models::tasklist::TaskType;
use models::interface::IsAdapter;
use models::signal_meta::SignalMeta;

use super::weather_adapter::WeatherAdapter;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeatherResponse {
    pub dateutc: String,
    pub tempf: f64,
    pub humidity: f64,
    pub baromrelin: f64,
    pub baromabsin: f64,
    pub winddir: f64,
    pub windspeedmph: f64,
    pub windgustmph: f64,
    pub rainratein: f64,
    pub dailyrainin: f64,
    pub solarradiation: f64,
    pub uv: f64,
}

impl WeatherResponse {
    pub fn iter(&self) -> WeatherResponseIterator<'_> {
        WeatherResponseIterator {
            inner: self,
            index: 0,
        }
    }
}

pub struct WeatherResponseIterator<'a> {
    inner: &'a WeatherResponse,
    index: u8,
}

impl<'a> Iterator for WeatherResponseIterator<'a> {
    type Item = &'a f64;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => &self.inner.tempf,
            1 => &self.inner.humidity,
            2 => &self.inner.baromrelin,
            3 => &self.inner.baromabsin,
            4 => &self.inner.winddir,
            5 => &self.inner.windspeedmph,
            6 => &self.inner.windgustmph,
            7 => &self.inner.rainratein,
            8 => &self.inner.rainratein,
            9 => &self.inner.solarradiation,
            10 => &self.inner.uv,
            _ => return None,
        };
        self.index += 1;
        Some(ret)
    }
}

impl WeatherResponse {
    pub fn to_ingestion_packet(self, meta_data: Vec<SignalMeta>) -> IngestionPacket {
        let mut data: Vec<Measurement> = Vec::new();
        let meters = self.iter();
        let sys_time_now = SystemTime::now();
        let ts = sys_time_now
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        for (meta, value) in zip(meta_data.iter(), meters) {
            data.push(Measurement {
                timestamp: ts as i64,
                uuid: meta.uuid.clone().unwrap(),
                value: *value,
            });
        }
        IngestionPacket { data }
    }
}
