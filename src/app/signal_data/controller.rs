use std::ops::Bound;

use crate::sdb::SDBRepository;
use crate::app::signal_data::model::*;
use crate::app::signal_meta::model::Signal;

impl SDBRepository{
    pub async fn ingest_data(&self, data: IngestionPacket) ->IngestionResponse{
        let mut data_it = data.data.into_iter();
        let mut success_data: Vec<Measurement> = Vec::new();
        let mut failed_data: Vec<Measurement> = Vec::new();
        let mut already_ingested: Vec<Measurement> = Vec::new();
        while let Some(dp) = data_it.next(){
            let ingest_response: Result<Option<DataPoint>, surrealdb::Error> =
                self.db.create((dp.uuid.clone(), dp.timestamp.clone()))
                    .content(DataPoint::from(&dp)).await;
            match ingest_response{
                Ok(p) => success_data.push(dp),
                Err(msg) => {if msg.to_string().ends_with("already exists"){
                    already_ingested.push(dp);
                } else {
                    failed_data.push(dp);
                }}
            };
        }
        if failed_data.is_empty() && already_ingested.is_empty() {
            IngestionResponse::Success
        } else {
        IngestionResponse::MultiStatus(MultiStatusData { success: success_data,
            failed: failed_data, already_exists: already_ingested })
        }
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
                        data: result
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
