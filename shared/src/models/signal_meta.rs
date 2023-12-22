use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SignalMeta{
    pub name: String,
    pub uuid: Option<String>,
    pub interface_uuid: Option<String>,
    pub uom: String,
    pub uom_symbol: String
}

impl SignalMeta {
    pub fn add_uuid(&mut self){
        self.uuid = Some(Uuid::new_v4().to_string());
    }
}

