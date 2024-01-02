use crate::models::shelly_v1::IsSignalResponse;
use hdc_shared::models::{ingestion_container::IngestionPacket, tasklist::CollectorTask};
use reqwest::{self, Error, StatusCode};
use serde::de::DeserializeOwned;
use log::{error, info, warn};
use tokio::runtime::Runtime;
use std::time::Duration;

use timeseries_data::buffer_agent_client::BufferAgentClient;
use timeseries_data::{BufferRequest, BufferResponse};

pub mod timeseries_data {
    tonic::include_proto!("timeseries_buffer");
}

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
        .send()
        .unwrap();
    match response.status() {
        StatusCode::OK => (), 
        _ => {
            error!("Ingestion failed, backend unreachable");
            let rt = Runtime::new().unwrap();
            rt.block_on(async move {
                let buffer_response = send_data_to_buffer(&ingestion_body).await;
                match buffer_response {
                    Ok(_) => info!("Sent failed ingestiondata to the BufferAgent"),
                    Err(_) => {
                        let mut retry_count: i32 = 0;
                        warn!("Unable to reach BufferAgent, retrying in 10s");
                        std::thread::sleep(Duration::from_secs(10));
                    }
                }
            });
            ()
        },
        };

    Ok(()) 
}

async fn send_data_to_buffer(ingestion_data: &IngestionPacket) -> Result<(), Box<dyn std::error::Error>>{
    let mut client = BufferAgentClient::connect("http://[::1]:50051").await?;
    
    let request = tonic::Request::new(
        BufferRequest{data: serde_json::to_vec(ingestion_data).unwrap()}
        );
    let response = client.send_timeseries_buffer(request).await?;
    Ok(())
}
