use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Ws, Client};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use serde::{Deserialize, Serialize};

use crate::model::sensor::Sensor;



pub struct SDBRepository {
    db: Surreal<Client>,
}

pub struct SDBError;

pub struct Record {
    id: Thing
}

impl SDBRepository {
    pub fn init(client: Surreal<Client>) -> SDBRepository {
        SDBRepository{
            db: client
        }
    }

    pub async fn register_sensor(&self, sensor: Sensor) -> Result<(), SDBError>{
        let created: Result<Vec<Sensor>, surrealdb::Error> = self.db.create("sensor").content(sensor).await;
        match created {
            Ok(_) => Ok(()),
            Err(_) => Err(SDBError),
        }
    }
}