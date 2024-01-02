use crate::models::shelly_v1::IsSignalResponse;
use hdc_shared::models::{ingestion_container::IngestionPacket, tasklist::CollectorTask};
use reqwest::{self, Error};
use serde::de::DeserializeOwned;
use log::error;

pub fn extract<S>(task: CollectorTask) -> Result<(), Error>
where
    S: DeserializeOwned + IsSignalResponse,
{
    let client = reqwest::blocking::Client::new();
    let body = client.get(task.url).send()?.json::<S>();

    let ingestion_body: IngestionPacket = match body {
        Ok(response) => response.to_ingestion_packet(task.signals),
        Err(e) => return Err(e),
    };
    let response = client
        .post("http://127.0.0.1:8080/v1/ingest")
        .body(serde_json::to_string(&ingestion_body).unwrap())
        .send();
    match response {
        Ok(_) => (), //return ok to the scheduler
        Err(_) => {
            error!("Ingestion failed, backend unreachable"); //add logic to put the ingestion_body to the buffer_handler
            ()
        },
        };

    Ok(()) //even if response has error return is Ok since the problem get propagated to the buffer
           //handler
}
