use super::signal_meta::*;
use crate::models::interface::IsAdapter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherAdapter {
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
    pub humidity: SignalMeta,
}

impl IsAdapter for WeatherAdapter {
    fn add_uuid(&mut self) {
        self.temp.add_uuid();
        self.dewpoint.add_uuid();
        self.windchill.add_uuid();
        self.windspeed.add_uuid();
        self.windgust.add_uuid();
        self.pressure.add_uuid();
        self.preciprate.add_uuid();
        self.preciptotal.add_uuid();
        self.solar_radiation.add_uuid();
        self.uv.add_uuid();
        self.wind_dir.add_uuid();
        self.humidity.add_uuid();
    }
    fn get_signals(&self) -> Vec<SignalMeta> {
        let mut signal: Vec<SignalMeta> = Vec::new();
        signal.push(self.temp.clone());
        signal.push(self.dewpoint.clone());
        signal.push(self.windchill.clone());
        signal.push(self.windspeed.clone());
        signal.push(self.windgust.clone());
        signal.push(self.pressure.clone());
        signal.push(self.preciprate.clone());
        signal.push(self.preciptotal.clone());
        signal.push(self.solar_radiation.clone());
        signal.push(self.uv.clone());
        signal.push(self.wind_dir.clone());
        signal.push(self.humidity.clone());
        signal
    }
}
