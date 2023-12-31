use uuid::Uuid;
use std::io::{self, Error};
use serde::{Serialize, Deserialize};
use super::signal_meta::*;

#[derive(Serialize, Deserialize)]
pub struct ShellyV2Adapter{
    pub temp_100: SignalMeta,
    pub temp_101: SignalMeta,
    pub temp_102: SignalMeta,
    pub temp_103: SignalMeta
}

impl ShellyV2Adapter{
    pub fn add_uuid(&mut self, interface_uuid: &String){
        self.temp_100.add_uuid(interface_uuid);
        self.temp_101.add_uuid(interface_uuid);
        self.temp_102.add_uuid(interface_uuid);
        self.temp_103.add_uuid(interface_uuid);
    }
    pub fn get_signals(&self) -> Vec<SignalMeta>{
        let mut signals: Vec<SignalMeta> = Vec::new();
        signals.push(self.temp_100.clone());
        signals.push(self.temp_101.clone());
        signals.push(self.temp_102.clone());
        signals.push(self.temp_103.clone());
        signals
    }
}
