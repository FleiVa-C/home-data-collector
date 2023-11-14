use std::task::Wake;

use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Ws, Client};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use serde::{Deserialize, Serialize};
use std::ops::Bound;

use crate::api::signal::SignalIdentifier;
use crate::model::query_timeseries::QueryTimeseriesData;
use crate::model::signal::Signal;
use crate::model::ingest_packet::{IngestionPacket, DataPoint};



pub struct SDBRepository {
    db: Surreal<Client>,
}

pub enum IngestionResponse{
    Success,
    Failed(IngestionPacket)
}

#[derive(Serialize, Deserialize)]
pub struct DataValue {
    timestamp: i64,
    value: f64
}

#[derive(Serialize, Deserialize)]
pub struct SignalData {
    signal_uuid: String,
    signal_name: String,
    data: Vec<DataValue>,
    uom: String,
    display_uom: String
}

#[derive(Serialize, Deserialize)]
pub struct QueryResult {
    data: Vec<SignalData>
}

pub enum QueryResponse{
    Success(QueryResult),
    Failed
}

pub struct SDBError;

impl SDBRepository {
    pub fn init(client: Surreal<Client>) -> Self {
        SDBRepository{
            db: client
        }
    }

    pub async fn register_signal(&self, signal: Signal) -> Result<(), SDBError>{
        let created: Result<Option<Signal>, surrealdb::Error> =
            self.db.create(("signal", signal.get_global_id())).content(signal).await;
        match created {
            Ok(_) => Ok(()),
            Err(_) => Err(SDBError),
        }
    }

    pub async fn get_signal(&self, signal: SignalIdentifier) -> Option<Signal>{
        let response: Result<Option<Signal>, surrealdb::Error> =
            self.db.select(("signal", signal.get_global_id())).await;
        match response {
            Ok(output) => output,
            Err(_) => None,
        }sdb
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
            let ingest_response: Result<Option<DataPoint>, surrealdb::Error> =
                self.db.create((dp.suuid.clone(), dp.timestamp.clone())).content(dp.clone()).await;
            match ingest_response{
                Ok(p) => (),
                Err(_) => { let mut failed_data: Vec<DataPoint> = data_it.collect();
                    failed_data.insert(0, dp);
                    return IngestionResponse::Failed(IngestionPacket{data: failed_data})}
            }
        }
        IngestionResponse::Success
    }
    pub async fn query_timeseries(&self, data: QueryTimeseriesData) -> QueryResponse{
        let mut response_data:Vec<SignalData> = Vec::new();
        let mut query = data.signals.into_iter();

        while let Some(signal) = query.next(){
            let signal_query: Result<Option<Signal>, surrealdb::Error> =
                self.db.select(("signal", &signal)).await;
            let signal_response = match signal_query {
                Ok(response) => response.unwrap(),
                Err(_) => return QueryResponse::Failed
            };
            let ts_query: Result<Vec<DataPoint>, surrealdb::Error> =
                self.db.select(&signal).range((Bound::Included(data.time_from as i32),
                Bound::Included(data.time_to as i32))).await;

            match ts_query{
                Ok(result) => {
                    let response = SignalData{
                        signal_uuid: signal_response.uuid ,
                        signal_name: signal_response.name,
                        uom: signal_response.uom,
                        display_uom: signal_response.display_uom,
                        data: result.iter().map(|dp| DataValue {timestamp: dp.timestamp,
                            value: dp.value}).collect()
                    };

                    response_data.push(response);
                    },
                Err(_) => return QueryResponse::Failed
            }

        }
    let query_result = QueryResult{data: response_data};
    QueryResponse::Success(query_result)
    }

}
