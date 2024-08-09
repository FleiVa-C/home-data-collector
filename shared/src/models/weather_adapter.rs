use super::signal_meta::*;
use crate::models::interface::IsAdapter;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct WeatherAdapter {
    pub tempf: SignalMeta,
    pub humidity: SignalMeta,
    pub baromrelin: SignalMeta,
    pub baromabsin: SignalMeta,
    pub winddir: SignalMeta,
    pub windspeedmph: SignalMeta,
    pub windgustmph: SignalMeta,
    pub rainratein: SignalMeta,
    pub dailyrainin: SignalMeta,
    pub solarradiation: SignalMeta,
    pub uv: SignalMeta,
}

impl IsAdapter for WeatherAdapter {
    fn add_uuid(&mut self) {
        self.tempf.add_uuid();
        self.humidity.add_uuid();
        self.baromrelin.add_uuid();
        self.baromabsin.add_uuid();
        self.winddir.add_uuid();
        self.windspeedmph.add_uuid();
        self.windgustmph.add_uuid();
        self.rainratein.add_uuid();
        self.dailyrainin.add_uuid();
        self.solarradiation.add_uuid();
        self.uv.add_uuid();
    }
    fn get_signals(&self) -> Vec<SignalMeta> {
        let mut signal: Vec<SignalMeta> = Vec::new();
        signal.push(self.tempf.clone());
        signal.push(self.humidity.clone());
        signal.push(self.baromrelin.clone());
        signal.push(self.baromabsin.clone());
        signal.push(self.winddir.clone());
        signal.push(self.windspeedmph.clone());
        signal.push(self.windgustmph.clone());
        signal.push(self.rainratein.clone());
        signal.push(self.dailyrainin.clone());
        signal.push(self.dailyrainin.clone());
        signal.push(self.solarradiation.clone());
        signal.push(self.uv.clone());
        signal
    }
}

impl WeatherAdapter {
    pub fn iter(&self) -> WeatherAdapterIterator<'_> {
        WeatherAdapterIterator {
            inner: self,
            index: 0,
        }
    }
}

pub struct WeatherAdapterIterator<'a> {
    inner: &'a WeatherAdapter,
    index: u8,
}

impl<'a> Iterator for WeatherAdapterIterator<'a> {
    type Item = &'a SignalMeta;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => &self.inner.tempf,
            1 => &self.inner.humidity,
            2 => &self.inner.baromrelin,
            3 => &self.inner.baromabsin,
            4 => &self.inner.winddir,
            5 => &self.inner.windspeedmph,
            6 => &self.inner.windgustmph,
            7 => &self.inner.rainratein,
            8 => &self.inner.rainratein,
            9 => &self.inner.solarradiation,
            10 => &self.inner.uv,
            _ => return None,
        };
        self.index += 1;
        Some(ret)
    }
}
