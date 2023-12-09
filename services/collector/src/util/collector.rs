use reqwest::{self, Error};
use crate::models::shelly_v1::ShellyV1Response;
use hdc_shared::models::ingestion_container::*;

pub fn extract(url: String) -> Result<(), Error>{
    let client = reqwest::blocking::Client::new();
    let body = client.get(url)
        .send()?
        .json::<ShellyV1Response>();

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
