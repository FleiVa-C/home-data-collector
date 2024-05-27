use hdc_shared::models::ingestion_container::IngestionPacket;
use log::{error, info, warn};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Error;
use std::time::Duration;
use tokio::sync::mpsc::Receiver;
use tokio::sync::OnceCell;
use uuid::Uuid;
use zstd::{Decoder, Encoder};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BufferWrapper {
    uuid: String,
    packet: IngestionPacket,
}

macro_rules! skip_fail {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                warn!("An error: {}; skipped.", e);
                continue;
            }
        }
    };
}

pub async fn buffer_handler(
    path: &str,
    mut recv_channel: Receiver<IngestionPacket>,
) -> Result<(), Box<dyn std::error::Error>> {
    while let Some(data) = recv_channel.recv().await {
        let data_packet = BufferWrapper {
            uuid: Uuid::new_v4().to_string(),
            packet: data,
        };
        let mut retry_count: u8 = 0;
        let writer = File::create(format!("{}/{}.json.zstd", path, data_packet.uuid))?;
        let mut writer = Encoder::new(writer, 0)?.auto_finish();
        while retry_count < 3 {
            let success = serde_json::to_writer(&mut writer, &data_packet.packet);
            match success {
                Ok(_) => {
                    info!("buffer_handler: Data added to local buffer.");
                    break;
                }
                Err(_) => {
                    error!("buffer_handler: Data failed to add to local buffer, retrying in 5 seconds.");
                    retry_count += 1;
                    std::thread::sleep(Duration::from_secs(5));
                }
            }
        }
        if retry_count != 0 {
            error!(
                "buffer_handler: Failed to add Data to local buffer after {} retries, aborting.",
                retry_count
            );
        }
    }
    Ok(())
}

pub async fn buffer_ingestor(
    path: &str,
    ingestion_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(format!("{}", path))? {
        if let Ok(dir) = entry {
            let reader = skip_fail!(File::open(dir.path()));
            let reader = skip_fail!(Decoder::new(reader));
            let data: IngestionPacket = skip_fail!(serde_json::from_reader(reader));
            let client = reqwest::Client::new();
            let response = client
                .post(ingestion_url)
                .body(serde_json::to_string(&data).unwrap())
                .send()
                .await;
            match response {
                Ok(resp) => {
                    match resp.status() {
                        reqwest::StatusCode::OK => {
                            fs::remove_file(dir.path())?;
                        },
                        _ => warn!("buffer_ingestor: failed to ingest packet, will keep in buffer for next ingestion")
                    }
                },
                Err(_) => warn!("buffer_ingestor: failed to ingest packet, will keep in buffer for next ingestion")
            }
        };
    }
    Ok(())
}
