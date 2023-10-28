use serde::{Serialize, Deserialize};
use uuid::Uuid;
use surrealdb::sql::{Object, Value, Thing};
use std::collections::BTreeMap;
use std::fmt::Debug;
use surrealdb::opt::RecordId;

use crate::utils::macros::map;

#[derive(Serialize, Deserialize, Debug)]
pub struct Sensor {
    id: Option<RecordId>,
    uuid: String,
    name: String,
    uom: String,
}


impl Sensor {
    pub fn new(sensor_uuid: String, sensor_name: String, sensor_uom: String) -> Sensor {
        Sensor{
            id: None,
            uuid: sensor_uuid,
            name: sensor_name,
            uom: sensor_uom
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