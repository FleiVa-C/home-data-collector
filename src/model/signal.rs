use serde::{Serialize, Deserialize};
use uuid::Uuid;
use surrealdb::sql::{Object, Value, Thing};
use std::collections::BTreeMap;
use std::fmt::Debug;
use surrealdb::opt::RecordId;

use crate::utils::macros::map;

#[derive(Serialize, Deserialize, Debug)]
pub struct Signal {
    id: Option<RecordId>,
    uuid: String,
    name: String,
    uom: String,
}


impl Signal {
    pub fn new(signal_uuid: String, signal_name: String, signal_uom: String) -> Signal {
        Signal{
            id: None,
            uuid: signal_uuid,
            name: signal_name,
            uom: signal_uom
        }
    }

    pub fn get_global_id(&self) -> String {
        return format!("{}", self.uuid);
    }
}

impl From<Signal> for Value {
    fn from(val: Signal) -> Self {
        map![ "id".into() => val.uuid.into(),].into()
    }
}
