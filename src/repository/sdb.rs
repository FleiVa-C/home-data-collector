use std::task::Wake;

use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Ws, Client};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use serde::{Deserialize, Serialize};

use crate::api::signal::SignalIdentifier;
use crate::model::signal::Signal;
use crate::model::ingest_packet::{IngestionPacket, DataPoint};



pub struct SDBRepository {
    db: Surreal<Client>,
}

pub enum IngestionResponse{
    Success,
    Failed(IngestionPacket)
}

pub struct SDBError;

impl SDBRepository {
    pub fn init(client: Surreal<Client>) -> Self {
        SDBRepository{
            db: client
        }
    }

    pub async fn register_signal(&self, signal: Signal) -> Result<(), SDBError>{
        let created: Result<Option<Signal>, surrealdb::Error> = self.db.create(("signal", signal.get_global_id())).content(signal).await;
        match created {
            Ok(_) => Ok(()),
            Err(_) => Err(SDBError),
        }
    }

    pub async fn get_signal(&self, signal: SignalIdentifier) -> Option<Signal>{
        let response: Result<Option<Signal>, surrealdb::Error> = self.db.select(("signal", signal.get_global_id())).await;
        match response {
            Ok(output) => output,
            Err(_) => None,
        }
    }
    
    pub async fn get_all_signals(&self) -> Result<Vec<Signal>, SDBError>{
        let response_data: Result<Vec<Signal>, surrealdb::Error> = self.db.select("signal").await;
        match response_data {
            Ok(response_data) => Ok(response_data),
            Err(_) => Err(SDBError)
            }
    }

    pub async fn ingest_data(&self, data: IngestionPacket) ->IngestionResponse{
        let mut data_it = data.data.into_iter();
        while let Some(dp) = data_it.next(){
            let ingest_response: Result<Option<DataPoint>, surrealdb::Error> = self.db.create((dp.suuid.clone(), dp.timestamp.clone())).content(dp).await;
            match ingest_response{
                Ok(p) => (),
                Err(_) => return IngestionResponse::Failed(IngestionPacket{data: data_it.collect()})
            }
        }
        IngestionResponse::Success
    }
}
