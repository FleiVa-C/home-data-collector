use serde::{Serialize, Deserialize};
use super::signal_meta::*;

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
