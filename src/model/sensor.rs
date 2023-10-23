use serde::{Serialize, Deserialize};
use uuid::Uuid;
use surrealdb::sql::{Object, Value};
use std::collections::BTreeMap;

use crate::utils::macros::map;

#[derive(Serialize, Deserialize)]
pub struct Sensor {
    uuid: String
}


impl Sensor {
    pub fn new(sensor_uuid: String) -> Sensor {
        Sensor{
            uuid: sensor_uuid
        }
    }

    pub fn get_global_id(&self) -> String {
        return format!("{}", self.uuid);
    }
}

impl From<Sensor> for Value {
    fn from(val: Sensor) -> Self {
        map![ "id".into() => val.uuid.into(),].into()
    }
}