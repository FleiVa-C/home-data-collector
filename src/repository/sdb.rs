use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Ws, Client};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use serde::{Deserialize, Serialize};

use crate::api::sensor::SensorIdentifier;
use crate::model::sensor::Sensor;



pub struct SDBRepository {
    db: Surreal<Client>,
}

pub struct SDBError;

impl SDBRepository {
    pub fn init(client: Surreal<Client>) -> Self {
        SDBRepository{
            db: client
        }
    }

    pub async fn register_sensor(&self, sensor: Sensor) -> Result<(), SDBError>{
        let created: Result<Option<Sensor>, surrealdb::Error> = self.db.create(("sensor", sensor.get_global_id())).content(sensor).await;
        match created {
            Ok(_) => Ok(()),
            Err(_) => Err(SDBError),
        }
    }

    pub async fn get_sensor(&self, sensor: SensorIdentifier) -> Option<Sensor>{
        let response: Result<Option<Sensor>, surrealdb::Error> = self.db.select(("sensor", sensor.get_global_id())).await;
        match response {
            Ok(output) => output,
            Err(_) => None,
        }
    }
    
    pub async fn get_all_sensors(&self) -> Result<Vec<Sensor>, SDBError>{
        let response_data: Result<Vec<Sensor>, surrealdb::Error> = self.db.select("sensor").await;
        match response_data {
            Ok(response_data) => Ok(response_data),
            Err(_) => Err(SDBError)
            }
    }
}