use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

use hdc_shared::models::ingestion_container::*;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherResponse {
    pub observations: Vec<Observation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Observation {
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
    pub winddir: i64,
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

impl From<WeatherResponse> for IngestionPacket {
    fn from(value: WeatherResponse) -> Self {
        let obs = &value.observations[0];
        let ts: i64 = obs.epoch;
        let uuid: String = "weather_temp".to_string();
        let measurement_value: f64 = obs.metric.temp;
        IngestionPacket {
            data: vec![Measurement {
                timestamp: ts,
                uuid: uuid,
                value: measurement_value,
            }],
        }
    }
}
