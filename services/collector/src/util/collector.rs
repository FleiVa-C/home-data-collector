use hdc_shared::models::tasklist::CollectorTask;
use hdc_shared::models::ingestion_container::IngestionPacket;
use super::super::models::shelly_v1::*;
use reqwest::{self, Error};
use serde::de::DeserializeOwned;
use std::fmt::Debug;

pub fn extract<S>(task: CollectorTask) -> Result<(), Error>
where
    S: DeserializeOwned + IsSignalResponse + Debug
{
    let client = reqwest::blocking::Client::new();
    let body = client.get(task.url).send()?.json::<S>();

    let ingestion_body: IngestionPacket = match body {
        Ok(response) => response.to_ingestion_packet(task.signals),
        Err(e) => return Err(e),
    };
    let _ = client
        .post("http://127.0.0.1:8080/v1/ingest")
        .body(serde_json::to_string(&ingestion_body).unwrap())
        .send()
        .unwrap();
    Ok(())
}
