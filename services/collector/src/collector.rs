use crate::models::shelly_v1::IsSignalResponse;
use hdc_shared::models::{ingestion_container::IngestionPacket, tasklist::CollectorTask};
use reqwest::{self, Error, StatusCode};
use serde::de::DeserializeOwned;
use log::{error, info, warn};
use tokio::runtime::Runtime;
use std::time::Duration;
use std::sync::mpsc::Sender;

pub async fn collect<S>(task: CollectorTask,
                        ingestion_url: &str,
                        sender_channel: Sender<IngestionPacket>) -> Result<(), Error>
where
    S: DeserializeOwned + IsSignalResponse,
{
    let client = reqwest::Client::new();
    let body = reqwest::get(&task.url)
        .await?
        .json::<S>()
        .await?;

    let ingestion_body: IngestionPacket = body.to_ingestion_packet(task.signals);
    let response = client.post(ingestion_url)
        .body(serde_json::to_string(&ingestion_body).unwrap())
        .send()
        .await.unwrap();
    match response.status() {
        StatusCode::OK => {
            info!("Sucessfully ingested data: {}@{}",ingestion_body.data[0].timestamp, task.url)
        }, 
        _ => {
            warn!("Ingestion failed, backend unreachable");
            let mut retry_count: u64 = 0;
            loop{
                let buffer_response = sender_channel.send(ingestion_body.clone());
                match buffer_response {
                    Ok(_) => {
                        info!("Sent failed ingestiondata to the buffer.");
                        break
                    },
                    Err(_) => {
                        if retry_count < 5{
                            retry_count+= 1;
                            warn!("Unable to reach BufferAgent, retrying in 10s.");
                            std::thread::sleep(Duration::from_secs(10));
                        }else{
                            error!("Retried for 5 times, aborting buffering");
                            break
                        }
                    }
                }
            }
            ()
        },
        };

    Ok(()) 
}
