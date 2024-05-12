use super::shelly_v1_adapter::ShellyV1Adapter;
use super::shelly_v2_adapter::ShellyV2Adapter;
use super::signal_meta::SignalMeta;
use super::weather_adapter::WeatherAdapter;
use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "interface_type")]
pub enum InterfaceModel {
    ShellyV1(Interface<ShellyV1Adapter>),
    ShellyV2(Interface<ShellyV2Adapter>),
    WeatherAPI(Interface<WeatherAdapter>),
}

impl InterfaceModel {
    pub fn get_url(&self) -> String {
        match self {
            Self::ShellyV1(value) => value.url.clone(),
            Self::ShellyV2(value) => value.url.clone(),
            Self::WeatherAPI(value) => value.url.clone(),
        }
    }
    pub fn add_uuid(&mut self){
        match self {
            Self::ShellyV1(value) => value.add_uuid(),
            Self::ShellyV2(value) => value.add_uuid(),
            Self::WeatherAPI(value) => value.add_uuid(),
        }
    }
    pub fn get_signals(&self) -> Vec<SignalMeta>{
        match self {
            Self::ShellyV1(value) => value.signals.get_signals(),
            Self::ShellyV2(value) => value.signals.get_signals(),
            Self::WeatherAPI(value) => value.signals.get_signals(),
        }
    }
    pub fn get_uuid(&self) -> Option<String> {
        match self {
            Self::ShellyV1(value) => value.uuid.clone(),
            Self::ShellyV2(value) => value.uuid.clone(),
            Self::WeatherAPI(value) => value.uuid.clone(),
        }
    }
        
}

pub trait IsAdapter{
    fn add_uuid(&mut self);
    fn get_signals(&self)-> Vec<SignalMeta>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Interface<T> where T: IsAdapter{
    pub uuid: Option<String>,
    pub name: String,
    pub url: String,
    pub signals: T,
}

impl <T:IsAdapter> Interface<T> {
    pub fn add_uuid(&mut self) {
        self.uuid = Some(Uuid::new_v4().to_string());
        self.signals.add_uuid()
    }
    pub fn get_global_id(&self) -> &Option<String> {
        &self.uuid
    }
}
