use uuid::Uuid;
use std::io::{self, Error};
use serde::{Serialize, Deserialize};
use super::signal_meta::*;

#[derive(Serialize, Deserialize)]
pub enum InterfaceType {
    ShellyV1,
    ShellyV2,
    WeatherAPI,
}

impl From<SignalMeta> for String {
    fn from(value: SignalMeta) -> Self {
        value.uuid.unwrap()
    }
}

impl From<ShellyV1Adapter> for ShellyV1AdapterLight {
    fn from(value: ShellyV1Adapter) -> Self {
        ShellyV1AdapterLight{
            emeter_1: String::from(value.emeter_1),
            emeter_2: String::from(value.emeter_2),
            emeter_3: String::from(value.emeter_3),
            emeter_4: String::from(value.emeter_4) 
        }
    }
    
}

impl From<ShellyV2Adapter> for ShellyV2AdapterLight{
    fn from(value: ShellyV2Adapter) -> ShellyV2AdapterLight {
        ShellyV2AdapterLight{
            temp_100: String::from(value.temp_100),
            temp_101: String::from(value.temp_101),
            temp_102: String::from(value.temp_102),
            temp_103: String::from(value.temp_103) 
        }
    }
}

impl From<WeatherAdapter> for WeatherAdapterLight{
    fn from(value: WeatherAdapter) -> WeatherAdapterLight {
        WeatherAdapterLight{
            temp: String::from(value.temp),
            dewpoint: String::from(value.dewpoint),
            windchill: String::from(value.windchill),
            windspeed: String::from(value.windspeed),
            windgust: String::from(value.windgust),
            pressure: String::from(value.pressure),
            preciprate: String::from(value.preciprate),
            preciptotal: String::from(value.preciptotal)
        }
    }
}

pub trait IsAdapter{}

impl IsAdapter for ShellyV1Adapter{}

impl IsAdapter for ShellyV2Adapter{}
impl IsAdapter for WeatherAdapter{}

impl Interface<ShellyV1Adapter>{
    fn make_light(self) -> InterfaceLight<ShellyV1AdapterLight>{
        InterfaceLight{
            base_url: self.base_url,
            interface_type: self.interface_type.unwrap(),
            signals: ShellyV1AdapterLight::from(self.signals)
        }
    }
}

impl Interface<ShellyV2Adapter>{
    fn make_light(self) -> InterfaceLight<ShellyV2AdapterLight>{
        InterfaceLight{
            base_url: self.base_url,
            interface_type: self.interface_type.unwrap(),
            signals: ShellyV2AdapterLight::from(self.signals)
        }
    }
}

impl Interface<WeatherAdapter>{
    fn make_light(self) -> InterfaceLight<WeatherAdapterLight>{
        InterfaceLight{
            base_url: self.base_url,
            interface_type: self.interface_type.unwrap(),
            signals: WeatherAdapterLight::from(self.signals)
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Interface<T: IsAdapter> {
    pub uuid: Option<String>,
    pub name : String,
    pub base_url: String,
    pub interface_type: Option<InterfaceType>,
    pub signals: T,
}

pub struct InterfaceLight<T: IsLightAdapter> {
    pub base_url: String,
    pub interface_type: InterfaceType,
    pub signals: T,
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
    pub preciptotal: SignalMeta
}

pub trait IsLightAdapter{}
impl IsLightAdapter for ShellyV1AdapterLight{}
impl IsLightAdapter for ShellyV2AdapterLight{}
impl IsLightAdapter for WeatherAdapterLight{}

#[derive(Serialize, Deserialize)]
pub struct ShellyV1AdapterLight{
        pub emeter_1: String,
        pub emeter_2: String,
        pub emeter_3: String,
        pub emeter_4: String
}

#[derive(Serialize, Deserialize)]
pub struct ShellyV2AdapterLight{
    pub temp_100: String,
    pub temp_101: String,
    pub temp_102: String,
    pub temp_103: String
}

#[derive(Serialize, Deserialize)]
pub struct WeatherAdapterLight{
    pub temp: String,
    pub dewpoint: String,
    pub windchill: String,
    pub windspeed: String,
    pub windgust: String,
    pub pressure: String,
    pub preciprate: String,
    pub preciptotal: String
}

impl Interface<ShellyV1Adapter> {
    pub fn add_uuids(&mut self){
        self.uuid = Some(Uuid::new_v4().to_string());
        self.signals.add_uuid();
    }
    pub fn add_interface_type(&mut self, interface_type: InterfaceType){
        self.interface_type = Some(interface_type);
    }
    pub fn get_global_id(&self) -> &Option<String>{
        &self.uuid
    }
}


impl ShellyV1Adapter{
    pub fn add_uuid(&mut self) {
        self.emeter_1.add_uuid();
        self.emeter_2.add_uuid();
        self.emeter_3.add_uuid();
        self.emeter_4.add_uuid();
    }
}
