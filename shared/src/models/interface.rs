use uuid::Uuid;
use serde::{Serialize, Deserialize};
use super::signal_meta::SignalMeta;
use super::shelly_v1_adapter::ShellyV1Adapter;
use super::shelly_v2_adapter::ShellyV2Adapter;
use super::weather_adapter::WeatherAdapter;

#[derive(Serialize, Deserialize, Debug)]
pub enum InterfaceType {
    ShellyV1,
    ShellyV2,
    WeatherAPI
}

#[derive(Serialize, Deserialize)]
pub enum AdapterType {
    ShellyV1(ShellyV1Adapter),
    ShellyV2(ShellyV2Adapter),
    WeatherAPI(WeatherAdapter)
}

impl AdapterType{
    pub fn add_uuid(&mut self, interface_uuid: String){
        match self{
            Self::ShellyV1(adapter) => adapter.add_uuid(&interface_uuid),
            Self::ShellyV2(adapter) => adapter.add_uuid(&interface_uuid),
            Self::WeatherAPI(adapter) => adapter.add_uuid(&interface_uuid),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Interface{
    pub uuid: Option<String>,
    pub name : String,
    pub interface_type : Option<InterfaceType>,
    pub base_url: String,
    pub signals: AdapterType,
}


impl Interface {
    pub fn add_uuid(&mut self){
        self.uuid = Some(Uuid::new_v4().to_string());
        self.signals.add_uuid(self.uuid.clone().unwrap()) 
    }
    pub fn add_interface_type(&mut self, interface_type: InterfaceType){
        self.interface_type = Some(interface_type);
    }
    pub fn get_global_id(&self) -> &Option<String>{
        &self.uuid
    }
}

