use std::ops::Bound;

use crate::sdb::SDBRepository;
use crate::app::signal_data::model::*;
use crate::app::signal_meta::model::Signal;

impl SDBRepository{
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
