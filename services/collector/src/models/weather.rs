use serde_derive::Deserialize;
use serde_derive::Serialize;

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
    pub tz: String,
    pub obs_time_utc: String,
    pub obs_time_local: String,
    pub epoch: i64,
    pub lat: f64,
    pub lon: f64,
    pub solar_radiation_high: f64,
    pub uv_high: f64,
    pub winddir_avg: i64,
    pub humidity_high: f64,
    pub humidity_low: f64,
    pub humidity_avg: f64,
    pub qc_status: i64,
    pub metric: Metric,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metric {
    pub temp_high: f64,
    pub temp_low: f64,
    pub temp_avg: f64,
    pub windspeed_high: f64,
    pub windspeed_low: f64,
    pub windspeed_avg: f64,
    pub windgust_high: f64,
    pub windgust_low: f64,
    pub windgust_avg: f64,
    pub dewpt_high: f64,
    pub dewpt_low: f64,
    pub dewpt_avg: f64,
    pub windchill_high: f64,
    pub windchill_low: f64,
    pub windchill_avg: f64,
    pub heatindex_high: f64,
    pub heatindex_low: f64,
    pub heatindex_avg: f64,
    pub pressure_max: f64,
    pub pressure_min: f64,
    pub pressure_trend: f64,
    pub precip_rate: f64,
    pub precip_total: f64,
}
