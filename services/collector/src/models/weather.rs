use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use std::iter::zip;

use hdc_shared::models as models;
use models::ingestion_container::*;
use models::tasklist::TaskType;
use models::weather_adapter_light::WeatherAdapterLight;
use super::shelly_v1::IsSignalResponse;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherResponse {
    #[serde(rename = "stationID")]
    pub station_id: String,
    pub obs_time_utc: String,
    pub obs_time_local: String,
    pub neighborhood: String,
    pub software_type: String,
    pub country: String,
    pub solar_radiation: f64,
    pub lon: f64,
    pub realtime_frequency: Value,
    pub epoch: i64,
    pub lat: f64,
    pub uv: f64,
    pub winddir: f64,
    pub humidity: f64,
    pub qc_status: i64,
    pub metric: Metric,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metric {
    pub temp: f64,
    pub heat_index: f64,
    pub dewpt: f64,
    pub wind_chill: f64,
    pub wind_speed: f64,
    pub wind_gust: f64,
    pub pressure: f64,
    pub precip_rate: f64,
    pub precip_total: f64,
    pub elev: f64,
}

impl WeatherResponse{
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

impl <'a> Iterator for WeatherResponseIterator<'a> {
    type Item = &'a f64;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => &self.inner.metric.temp,
            1 => &self.inner.metric.dewpt,
            2 => &self.inner.metric.wind_chill,
            3 => &self.inner.metric.wind_speed,
            4 => &self.inner.metric.wind_gust,
            5 => &self.inner.metric.precip_rate,
            6 => &self.inner.metric.precip_total,
            7 => &self.inner.solar_radiation,
            8 => &self.inner.uv,
            9 => &self.inner.winddir,
            10 => &self.inner.humidity,
            _ => return None,
        };
        self.index += 1;
        Some(ret)
    }
}

impl IsSignalResponse for WeatherResponse{
    fn to_ingestion_packet(self, task_type: TaskType) -> IngestionPacket {
        let mut data: Vec<Measurement> = Vec::new();
        let meta_data: Option<WeatherAdapterLight> = match task_type{
            TaskType::WeatherTask(adapter) => Some(adapter),
            _ => None
        };
        let meters = self.iter();
        let emeters_uuid = meta_data.unwrap();
        let ts: i64 = self.epoch;
        for (uuid, value) in zip(emeters_uuid.iter(), meters){
            data.push(Measurement{
                timestamp: ts.clone(),
                uuid: uuid.clone(),
                value: *value,
            });
        };
        IngestionPacket {
            data
        }
    }
}
