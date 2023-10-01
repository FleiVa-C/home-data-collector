use serde::{Serialize, Deserialize};
use uuid::Uuid;

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