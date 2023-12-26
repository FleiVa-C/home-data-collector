use uuid::Uuid;
use std::io::{self, Error};
use serde::{Serialize, Deserialize};
use super::signal_meta::*;

#[derive(Serialize, Deserialize)]
pub struct ShellyV1Adapter{
        pub emeter_1: SignalMeta,
        pub emeter_2: SignalMeta,
        pub emeter_3: SignalMeta,
        pub emeter_4: SignalMeta
}

impl ShellyV1Adapter{
    pub fn add_uuid(&mut self, interface_uuid: &String) {
        self.emeter_1.add_uuid(interface_uuid);
        self.emeter_2.add_uuid(interface_uuid);
        self.emeter_3.add_uuid(interface_uuid);
        self.emeter_4.add_uuid(interface_uuid);
    }
}
