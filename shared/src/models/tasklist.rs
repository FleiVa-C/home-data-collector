use std::iter::Iterator;
use serde::{Serialize, Deserialize};
use super::interface::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskList {
    pub tasks: Vec<CollectorTask>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectorTask {
    pub url: String,
    pub interface_type: InterfaceType ,
    pub signals: TaskType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskType{
    ShellyV1Task(ShellyV1AdapterLight),
    ShellyV2Task(ShellyV2AdapterLight),
    WeatherTask(WeatherAdapterLight)
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ShellyV1AdapterLight{
        pub emeter_1: String,
        pub emeter_2: String,
        pub emeter_3: String,
        pub emeter_4: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShellyV2AdapterLight{
    pub temp_100: String,
    pub temp_101: String,
    pub temp_102: String,
    pub temp_103: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherAdapterLight{
    pub temp: String,
    pub dewpoint: String,
    pub windchill: String,
    pub windspeed: String,
    pub windgust: String,
    pub pressure: String,
    pub preciprate: String,
    pub preciptotal: String,
    pub solar_radiaton: String,
    pub uv: String,
    pub wind_dir: String,
    pub humidity: String
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
            preciptotal: String::from(value.preciptotal),
            solar_radiaton: String::from(value.solar_radiation),
            uv: String::from(value.uv),
            wind_dir: String::from(value.wind_dir),
            humidity: String::from(value.humidity),
        }
    }
}

impl From<Interface> for CollectorTask{
    fn from(value: Interface) -> Self {
        let adapter: TaskType = match value.signals{
            AdapterType::ShellyV1(adapter) => TaskType::ShellyV1Task(ShellyV1AdapterLight::from(adapter)),
            AdapterType::ShellyV2(adapter) => TaskType::ShellyV2Task(ShellyV2AdapterLight::from(adapter)),
            AdapterType::WeatherAPI(adapter) => TaskType::WeatherTask(WeatherAdapterLight::from(adapter)),
        };

        CollectorTask {
            url: value.base_url,
            interface_type: value.interface_type.unwrap(),
            signals: adapter }
    }
}

impl ShellyV1AdapterLight{
    pub fn iter(&self) -> ShellyV1Iterator<'_> {
        ShellyV1Iterator {
            inner: self,
            index: 0,
        }
    }
}

pub struct ShellyV1Iterator<'a> {
    inner: &'a ShellyV1AdapterLight,
    index: u8,
}

impl <'a> Iterator for ShellyV1Iterator<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => &self.inner.emeter_1,
            1 => &self.inner.emeter_2,
            2 => &self.inner.emeter_3,
            3 => &self.inner.emeter_4,
            _ => return None,
        };
        self.index += 1;
        Some(ret)
    }
}

impl ShellyV2AdapterLight{
    pub fn iter(&self) -> ShellyV2Iterator<'_> {
        ShellyV2Iterator {
            inner: self,
            index: 0,
        }
    }
}

pub struct ShellyV2Iterator<'a> {
    inner: &'a ShellyV2AdapterLight,
    index: u8,
}

impl <'a> Iterator for ShellyV2Iterator<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => &self.inner.temp_100,
            1 => &self.inner.temp_101,
            2 => &self.inner.temp_102,
            3 => &self.inner.temp_103,
            _ => return None,
        };
        self.index += 1;
        Some(ret)
    }
}

impl WeatherAdapterLight{
    pub fn iter(&self) -> WeatherIterator<'_> {
        WeatherIterator {
            inner: self,
            index: 0,
        }
    }
}

pub struct WeatherIterator<'a> {
    inner: &'a WeatherAdapterLight,
    index: u8,
}

impl <'a> Iterator for WeatherIterator<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => &self.inner.temp,
            1 => &self.inner.dewpoint,
            2 => &self.inner.windchill,
            3 => &self.inner.windspeed,
            4 => &self.inner.windgust,
            5 => &self.inner.preciprate,
            6 => &self.inner.preciptotal,
            7 => &self.inner.solar_radiaton,
            8 => &self.inner.uv,
            9 => &self.inner.wind_dir,
            10 => &self.inner.humidity,
            _ => return None,
        };
        self.index += 1;
        Some(ret)
    }
}
