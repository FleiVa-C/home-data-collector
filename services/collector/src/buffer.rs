use hdc_shared::models::ingestion_container::IngestionPacket;
use log::{error, info, warn};
use surrealdb::{engine::local::{File, Db}, Surreal};
use reqwest::Client;
use tokio::sync::OnceCell;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::sync::mpsc::Receiver;
use std::time::Duration;
use std::io::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BufferWrapper{
    uuid: String,
    packet: IngestionPacket
}

pub async fn buffer_handler(db_connection: &'static OnceCell<Surreal<Db>>,
                            recv_channel: Receiver<IngestionPacket>) -> () {
    while let Ok(data) = recv_channel.recv() {
        let data_packet = BufferWrapper{
            uuid: Uuid::new_v4().to_string(),
            packet: data
        };
        let db = db_connection.get().unwrap();
        let mut retry_count: u8 = 0;
        while retry_count < 3{
            let dp = data_packet.clone();
            let local_db_response: Result<Option<BufferWrapper>, surrealdb::Error> = db
                .create(("buffer_data", &dp.uuid))
                .content(dp)
                .await;
            match local_db_response {
                Ok(_) => {
                    info!("buffer_handler: Data added to local buffer.");
                    return;
                }
                Err(_) => {
                    error!("buffer_handler: Data failed to add to local buffer, retrying in 5 seconds.");
                    retry_count += 1;
                    std::thread::sleep(Duration::from_secs(5));
                }
            }
        }
        error!("buffer_handler: Failed to add Data to local buffer after {} retries, aborting.", retry_count);
    }
}

pub async fn buffer_ingestor(db_connection: &'static OnceCell<Surreal<Db>>,
                             ingestion_url: &str) -> Result<(), Error>{

let db = db_connection.get().unwrap();
    let local_db_response: Vec<BufferWrapper> = db
        .select("buffer_data")
        .await
        .map_err(|_| Error::other("cant read database"))?;

    if local_db_response.len() > 0 {
            let client = reqwest::Client::new();
            let mut data_it = local_db_response.iter();
            while let Some(data) = data_it.next(){
                let response = client.post(ingestion_url)
                    .body(serde_json::to_string(&data.packet).unwrap())
                    .send()
                    .await;
                match response {
                    Ok(resp) => {
                        match resp.status() {
                            reqwest::StatusCode::OK => {
                                let _: Option<BufferWrapper> = db_connection.get().unwrap()
                                    .delete(("buffer_data", &data.uuid))
                                    .await
                                    .map_err(|_| Error::other("cant delete"))?;
                            },
                            _ => warn!("buffer_ingestor: failed to ingest packet, will keep in buffer for next ingestion")

                        }
                    },
                    Err(_) => error!("buffer_ingestor: failed to ingest packet, will keep in buffer for next ingestion")
                }
            };
        }else{
            info!("buffer_ingestor: Buffer is empty.");
        };
    Ok(())

}
