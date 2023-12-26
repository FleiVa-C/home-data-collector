use serde_derive::Deserialize;
use serde_derive::Serialize;

use hdc_shared::models as models;
use models::ingestion_container::*;
use models::tasklist::TaskType;
use models::shelly_v2_adapter_light::ShellyV2AdapterLight;
use super::shelly_v1::IsSignalResponse;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellyV2Response {
    pub ble: Ble,
    pub cloud: Cloud,
    #[serde(rename = "input:0")]
    pub input_0: Input0,
    pub mqtt: Mqtt,
    #[serde(rename = "switch:0")]
    pub switch_0: Switch0,
    pub sys: Sys,
    #[serde(rename = "temperature:100")]
    pub temperature_100: Temperature100,
    #[serde(rename = "temperature:101")]
    pub temperature_101: Temperature101,
    #[serde(rename = "temperature:102")]
    pub temperature_102: Temperature102,
    #[serde(rename = "temperature:103")]
    pub temperature_103: Temperature103,
    pub wifi: Wifi,
    pub ws: Ws,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ble {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cloud {
    pub connected: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input0 {
    pub id: i64,
    pub state: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mqtt {
    pub connected: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Switch0 {
    pub id: i64,
    pub source: String,
    pub output: bool,
    pub apower: f64,
    pub voltage: f64,
    pub current: f64,
    pub aenergy: Aenergy,
    pub temperature: Temperature,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Aenergy {
    pub total: f64,
    #[serde(rename = "by_minute")]
    pub by_minute: Vec<f64>,
    #[serde(rename = "minute_ts")]
    pub minute_ts: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Temperature {
    pub t_c: f64,
    pub t_f: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sys {
    pub mac: String,
    #[serde(rename = "restart_required")]
    pub restart_required: bool,
    pub time: String,
    pub unixtime: i64,
    pub uptime: i64,
    #[serde(rename = "ram_size")]
    pub ram_size: i64,
    #[serde(rename = "ram_free")]
    pub ram_free: i64,
    #[serde(rename = "fs_size")]
    pub fs_size: i64,
    #[serde(rename = "fs_free")]
    pub fs_free: i64,
    #[serde(rename = "cfg_rev")]
    pub cfg_rev: i64,
    #[serde(rename = "kvs_rev")]
    pub kvs_rev: i64,
    #[serde(rename = "schedule_rev")]
    pub schedule_rev: i64,
    #[serde(rename = "webhook_rev")]
    pub webhook_rev: i64,
    #[serde(rename = "available_updates")]
    pub available_updates: AvailableUpdates,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableUpdates {
    pub beta: Beta,
    pub stable: Stable,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Beta {
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stable {
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Temperature100 {
    pub id: i64,
    pub t_c: f64,
    pub t_f: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Temperature101 {
    pub id: i64,
    pub t_c: f64,
    pub t_f: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Temperature102 {
    pub id: i64,
    pub t_c: f64,
    pub t_f: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Temperature103 {
    pub id: i64,
    pub t_c: f64,
    pub t_f: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wifi {
    #[serde(rename = "sta_ip")]
    pub sta_ip: String,
    pub status: String,
    pub ssid: String,
    pub rssi: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ws {
    pub connected: bool,
}

impl IsSignalResponse for ShellyV2Response{
    fn to_ingestion_packet(self, task_type: TaskType) -> IngestionPacket {
        let mut data: Vec<Measurement> = Vec::new();
        let meta_data: Option<ShellyV2AdapterLight> = match task_type{
            TaskType::ShellyV2Task(adapter) => Some(adapter),
            _ => None
        };
        let meta_data = meta_data.unwrap();
        let ts: i64 = self.sys.unixtime;
        data.push(Measurement{
            timestamp: ts.clone(),
            uuid: meta_data.temp_100,
            value: self.temperature_100.t_c,
        });
        data.push(Measurement{
            timestamp: ts.clone(),
            uuid: meta_data.temp_101,
            value: self.temperature_101.t_c,
        });
        data.push(Measurement{
            timestamp: ts.clone(),
            uuid: meta_data.temp_102,
            value: self.temperature_103.t_c,
        });
        IngestionPacket {
            data
        }
    }
}
