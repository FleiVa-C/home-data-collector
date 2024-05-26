#![allow(unused)]
use config::CollectorConfig;
use hdc_shared::utils::config::load_config;
use log::{warn, info};
use tokio::time::{interval_at, Duration, Instant};
use tokio::sync::{OnceCell, mpsc::channel};
use std::sync::{RwLock, Arc};
use std::time::SystemTime;

use hdc_shared::models::ingestion_container::IngestionPacket;
use hdc_shared::models::tasklist::Tasklist;
mod collector;
mod models;
mod task;
mod buffer;
mod config;

use task::{tasklist_observer, task_dispatcher};
use buffer::{buffer_handler, buffer_ingestor};

static TASKLIST: RwLock<Tasklist> = RwLock::new(Tasklist::new());

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let config: Arc<CollectorConfig> = Arc::new(CollectorConfig::load());

    let (send, mut recv) = channel::<IngestionPacket>(32);

    tasklist_observer(&TASKLIST, &config.tasklist_url).await;

    let sys_time_now = SystemTime::now();
    let start_offset = sys_time_now
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        % &config.collection_interval;

    let start_collecting = Instant::now() + Duration::from_secs((&config.collection_interval - start_offset).try_into().unwrap());

    let tasklist_observer_config = config.clone();
    tokio::spawn(
        async move {
            let mut interval = interval_at(start_collecting+Duration::from_secs(5)
                                           , Duration::from_secs(tasklist_observer_config.task_update_interval));
            loop {
                interval.tick().await;
                let tasklist_status = tasklist_observer(&TASKLIST, &tasklist_observer_config.tasklist_url).await;
                match tasklist_status {
                    Ok(()) => info!("Tasklist updated"),
                    Err(_) => warn!("Failed to get latest Tasklist")
                }
            }
        });

    let collector_config = config.clone();
    let path = format!("{}", &collector_config.db_path);
    let ingestion_url = Arc::new(collector_config.ingestion_url.clone());
    tokio::spawn(
        async move {
            let mut interval = interval_at(start_collecting, Duration::from_secs(collector_config.collection_interval));
            loop {
                interval.tick().await;
                task_dispatcher(&TASKLIST, ingestion_url.clone(), send.clone()).await;
            }
        });
    tokio::spawn(
        async move {
            buffer_handler(&path, recv).await;
        });

    
    let buffer_config = config.clone();
    tokio::spawn(
        async move {
            let mut interval = interval_at(start_collecting + Duration::from_secs(15), Duration::from_secs(buffer_config.buffer_ingestion_interval));
            loop {
                interval.tick().await;
                let status = buffer_ingestor(&buffer_config.db_path, &buffer_config.ingestion_url).await;
                match status {
                    Ok(()) => (),
                    Err(e) => warn!("bufer_ingestor: {:?}", e)
                }
            }
        });

    loop {}
}
