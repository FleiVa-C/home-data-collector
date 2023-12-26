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
}
