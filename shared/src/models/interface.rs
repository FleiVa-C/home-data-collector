use uuid::Uuid;
use std::io::{self, Error};
use serde::{Serialize, Deserialize};
use super::signal_meta::*;

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

#[derive(Serialize, Deserialize)]
pub struct ShellyV1Adapter{
        pub emeter_1: SignalMeta,
        pub emeter_2: SignalMeta,
        pub emeter_3: SignalMeta,
        pub emeter_4: SignalMeta
}

#[derive(Serialize, Deserialize)]
pub struct ShellyV2Adapter{
    pub temp_100: SignalMeta,
    pub temp_101: SignalMeta,
    pub temp_102: SignalMeta,
    pub temp_103: SignalMeta
}

#[derive(Serialize, Deserialize)]
pub struct WeatherAdapter{
    pub temp: SignalMeta,
    pub dewpoint: SignalMeta,
    pub windchill: SignalMeta,
    pub windspeed: SignalMeta,
    pub windgust: SignalMeta,
    pub pressure: SignalMeta,
    pub preciprate: SignalMeta,
    pub preciptotal: SignalMeta,
    pub solar_radiation: SignalMeta,
    pub uv: SignalMeta,
    pub wind_dir: SignalMeta,
    pub humidity: SignalMeta
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


impl ShellyV1Adapter{
    pub fn add_uuid(&mut self, interface_uuid: &String) {
        self.emeter_1.add_uuid(interface_uuid);
        self.emeter_2.add_uuid(interface_uuid);
        self.emeter_3.add_uuid(interface_uuid);
        self.emeter_4.add_uuid(interface_uuid);
    }
}


impl ShellyV2Adapter{
    pub fn add_uuid(&mut self, interface_uuid: &String){
        self.temp_100.add_uuid(interface_uuid);
        self.temp_101.add_uuid(interface_uuid);
        self.temp_102.add_uuid(interface_uuid);
        self.temp_103.add_uuid(interface_uuid);
    }
}

impl WeatherAdapter{
    pub fn add_uuid(&mut self, interface_uuid: &String){
        self.temp.add_uuid(interface_uuid);
        self.dewpoint.add_uuid(interface_uuid);
        self.windchill.add_uuid(interface_uuid);
        self.windspeed.add_uuid(interface_uuid);
        self.windgust.add_uuid(interface_uuid);
        self.pressure.add_uuid(interface_uuid);
        self.preciprate.add_uuid(interface_uuid);
        self.preciptotal.add_uuid(interface_uuid);
        self.solar_radiation.add_uuid(interface_uuid);
        self.uv.add_uuid(interface_uuid);
        self.wind_dir.add_uuid(interface_uuid);
        self.humidity.add_uuid(interface_uuid);
    }
}


impl From<SignalMeta> for String {
    fn from(value: SignalMeta) -> Self {
        value.uuid.unwrap()
    }
}
