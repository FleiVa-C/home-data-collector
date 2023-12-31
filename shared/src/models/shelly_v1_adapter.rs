use super::signal_meta::*;
use serde::{Deserialize, Serialize};
use std::io::{self, Error};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct ShellyV1Adapter {
    pub emeter_1: SignalMeta,
    pub emeter_2: SignalMeta,
    pub emeter_3: SignalMeta,
    pub emeter_4: SignalMeta,
}

impl ShellyV1Adapter {
    pub fn add_uuid(&mut self, interface_uuid: &String) {
        self.emeter_1.add_uuid(interface_uuid);
        self.emeter_2.add_uuid(interface_uuid);
        self.emeter_3.add_uuid(interface_uuid);
        self.emeter_4.add_uuid(interface_uuid);
    }
    pub fn get_signals(&self) -> Vec<SignalMeta> {
        let mut signal: Vec<SignalMeta> = Vec::new();
        signal.push(self.emeter_1.clone());
        signal.push(self.emeter_2.clone());
        signal.push(self.emeter_3.clone());
        signal.push(self.emeter_4.clone());
        signal
    }
}
