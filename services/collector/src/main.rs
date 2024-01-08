#![allow(unused)]
use tokio::time::{interval_at, Duration, Instant};
use tokio::sync::{OnceCell};
use std::sync::{mpsc::channel, RwLock};
use std::time::SystemTime;
use surrealdb::Surreal;
use surrealdb::engine::local::{File, Db};

use hdc_shared::models::ingestion_container::IngestionPacket;
use hdc_shared::models::tasklist::Tasklist;
mod collector;
mod models;
mod taskforce;
mod buffer;

use taskforce::{taskforce, tasklist_observer};
use buffer::{buffer_handler, buffer_ingestor};

static TASKLIST: RwLock<Tasklist> = RwLock::new(Tasklist::new());
static LOCAL_DB: OnceCell<Surreal<Db>> = OnceCell::const_new();
const DB_PATH: &str = "home/reberfla/test/temp.db";
const INGESTION_URL: &str = "http://127.0.0.1:8080/v1/ingest";
const TASKLIST_URL: &str = "http://127.0.0.1:8080/v1/get_tasks";
const COLLECTOR_INTERVAL: u64 = 30;
const TASK_UPDATE_INTERVAL: u64 = 300;
const BUFFER_INGESTION_INTERVAL: u64 = 600;

#[tokio::main(flavor = "multi_thread", worker_threads = 20)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    LOCAL_DB.get_or_init(|| async {
        let db = Surreal::new::<File>(DB_PATH).await.expect("cant connect to local db");
        db.use_ns("test").use_db("test").await;
        db
    }).await;

    let (send, recv) = channel::<IngestionPacket>();

    tasklist_observer(&TASKLIST, &TASKLIST_URL).await;

    let sys_time_now = SystemTime::now();
    let start_offset = sys_time_now
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        % COLLECTOR_INTERVAL;

    let start_collecting = Instant::now() + Duration::from_secs(COLLECTOR_INTERVAL - start_offset);

    tokio::spawn(
        async move {
            let mut interval = interval_at(start_collecting+Duration::from_secs(5)
                                           , Duration::from_secs(TASK_UPDATE_INTERVAL));
            loop {
                interval.tick().await;
                tasklist_observer(&TASKLIST, &TASKLIST_URL).await;
            }
        });

    tokio::spawn(
        async move {
            let mut interval = interval_at(start_collecting, Duration::from_secs(COLLECTOR_INTERVAL));
            loop {
                interval.tick().await;
                taskforce(&TASKLIST, &INGESTION_URL, send.clone()).await;
            }
        });
    tokio::spawn(
        async move {
            buffer_handler(&LOCAL_DB, recv).await;
        });

    tokio::spawn(
        async move {
            let mut interval = interval_at(start_collecting, Duration::from_secs(BUFFER_INGESTION_INTERVAL));
            loop {
                interval.tick().await;
                buffer_ingestor(&LOCAL_DB, &INGESTION_URL);
            }
        });

    loop {}
}
