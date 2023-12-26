use std::iter::zip;
use hdc_shared::models::tasklist::ShellyV1AdapterLight;
use serde_derive::Deserialize;
use serde_derive::Serialize;

use hdc_shared::models::ingestion_container::*;
use hdc_shared::models::tasklist::TaskType;

pub trait IsSignalResponse{
    fn to_ingestion_packet(self, task_type: TaskType) -> IngestionPacket;
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShellyV1Response {
    #[serde(rename = "wifi_sta")]
    pub wifi_sta: WifiSta,
    pub cloud: Cloud,
    pub mqtt: Mqtt,
    pub time: String,
    pub unixtime: i64,
    pub serial: i64,
    #[serde(rename = "has_update")]
    pub has_update: bool,
    pub mac: String,
    #[serde(rename = "cfg_changed_cnt")]
    pub cfg_changed_cnt: i64,
    #[serde(rename = "actions_stats")]
    pub actions_stats: ActionsStats,
    pub relays: Vec<Relay>,
    pub emeters: Vec<Emeter>,
    #[serde(rename = "total_power")]
    pub total_power: f64,
    #[serde(rename = "emeter_n")]
    pub emeter_n: EmeterN,
    #[serde(rename = "fs_mounted")]
    pub fs_mounted: bool,
    #[serde(rename = "v_data")]
    pub v_data: i64,
    #[serde(rename = "ct_calst")]
    pub ct_calst: i64,
    pub update: Update,
    #[serde(rename = "ram_total")]
    pub ram_total: i64,
    #[serde(rename = "ram_free")]
    pub ram_free: i64,
    #[serde(rename = "fs_size")]
    pub fs_size: i64,
    #[serde(rename = "fs_free")]
    pub fs_free: i64,
    pub uptime: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WifiSta {
    pub connected: bool,
    pub ssid: String,
    pub ip: String,
    pub rssi: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cloud {
    pub enabled: bool,
    pub connected: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mqtt {
    pub connected: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionsStats {
    pub skipped: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relay {
    pub ison: bool,
    #[serde(rename = "has_timer")]
    pub has_timer: bool,
    #[serde(rename = "timer_started")]
    pub timer_started: i64,
    #[serde(rename = "timer_duration")]
    pub timer_duration: i64,
    #[serde(rename = "timer_remaining")]
    pub timer_remaining: i64,
    pub overpower: bool,
    #[serde(rename = "is_valid")]
    pub is_valid: bool,
    pub source: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Emeter {
    pub power: f64,
    pub pf: f64,
    pub current: f64,
    pub voltage: f64,
    #[serde(rename = "is_valid")]
    pub is_valid: bool,
    pub total: f64,
    #[serde(rename = "total_returned")]
    pub total_returned: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmeterN {
    pub current: f64,
    pub ixsum: f64,
    pub mismatch: bool,
    #[serde(rename = "is_valid")]
    pub is_valid: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Update {
    pub status: String,
    #[serde(rename = "has_update")]
    pub has_update: bool,
    #[serde(rename = "new_version")]
    pub new_version: String,
    #[serde(rename = "old_version")]
    pub old_version: String,
    #[serde(rename = "beta_version")]
    pub beta_version: String,
}

impl IsSignalResponse for ShellyV1Response{
    fn to_ingestion_packet(self, task_type: TaskType) -> IngestionPacket {
        let mut data: Vec<Measurement> = Vec::new();
        let meta_data: Option<ShellyV1AdapterLight> = match task_type{
            TaskType::ShellyV1Task(adapter) => Some(adapter),
            _ => None
        };
        let emeters = self.emeters.into_iter();
        let emeters_uuid = meta_data.unwrap();
        let ts: i64 = self.unixtime;
        for (uuid, value) in zip(emeters_uuid.iter(), emeters){
            data.push(Measurement{
                timestamp: ts.clone(),
                uuid: uuid.clone(),
                value: value.power,
            });
        };
        data.push(Measurement{
            timestamp: ts.clone(),
            uuid: emeters_uuid.emeter_4.clone(),
            value: self.total_power,
        });
        IngestionPacket {
            data
        }
    }
}
