use serde::{Serialize, Deserialize};
use super::scheduler::Task;
use reqwest::{self, Error};
use serde::de::DeserializeOwned;
use crate::models::{shelly_v1::ShellyV1Response, shelly_v2::ShellyV2Response, weather::WeatherResponse};
use hdc_shared::models::ingestion_container::*;

pub fn extract<S>(task: Task) -> Result<(), Error>
    where S: IsSignal + DeserializeOwned + Into<IngestionPacket>
{
    let client = reqwest::blocking::Client::new();
    let body = client.get(task.url)
        .send()?
        .json::<S>();

    let ingestion_body: IngestionPacket = match body {
        Ok(response) => response.into(),
        Err(e) => return Err(e)
    };
    let _ = client
        .post("http://127.0.0.1:8080/v1/ingest")
        .body(serde_json::to_string(&ingestion_body).unwrap())
        .send()
        .unwrap();

    Ok(())
}

pub trait IsSignal {}

impl IsSignal for ShellyV1Response{}
impl IsSignal for ShellyV2Response{}
impl IsSignal for WeatherResponse{}


#[derive(Serialize, Deserialize)]
pub enum SensorType{
    ShellyV1,
    ShellyV2,
    WeatherAPI
}
