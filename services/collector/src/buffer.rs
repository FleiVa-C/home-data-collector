use hdc_shared::models::ingestion_container::IngestionPacket;
use log::{error, info};
use surrealdb::{engine::local::{File, Db}, Surreal};
use reqwest::Client;
use tokio::sync::OnceCell;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use std::sync::mpsc::Receiver;
use std::io::Error;

#[derive(Serialize, Deserialize)]
struct BufferWrapper{
    uid: String,
    packet: IngestionPacket
}

pub async fn buffer_handler(db_connection: &OnceCell<Surreal<Db>>,
                            recv_channel: Receiver<IngestionPacket>) -> () {
    while let Ok(data) = recv_channel.recv() {
        let data_packet = BufferWrapper{
            uid: Uuid::new_v4().to_string(),
            packet: data
        };
        let local_db_response: Result<Option<BufferWrapper>, surrealdb::Error> = db_connection.get().unwrap()
            .create(("buffer_data", &data_packet.uid))
            .content(data_packet)
            .await;
        match local_db_response {
            Ok(_) => info!("Data added to local buffer."),
            Err(e) => {
                error!("Data failed to add to local buffer");
                println!("{:?}", e) 
            }
        }
    }
}

pub async fn buffer_ingestor(db_connection: &OnceCell<Surreal<Db>>,
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
                    Ok(_) => {
                        let _: Option<BufferWrapper> = db_connection.get().unwrap()
                            .delete(("buffer_data", &data.uid))
                            .await
                            .map_err(|_| Error::other("cant delete"))?;
                    },
                    Err(_) => ()
                }
            };
        }else{
            info!("Buffer is empty.");
        };
    Ok(())

}
