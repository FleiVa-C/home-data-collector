use log::{error, info};
use std::time::{Duration, SystemTime};
use std::sync::{RwLock, Arc};
use tokio::sync::mpsc::Sender;

use super::collector::collect;
use crate::models::shelly_v1::ShellyV1Response;
use crate::models::shelly_v2::ShellyV2Response;
use crate::models::weather::WeatherResponse;
use hdc_shared::models::ingestion_container::IngestionPacket;
use hdc_shared::models::interface::InterfaceType;
use hdc_shared::models::tasklist::Tasklist;

pub async fn task_dispatcher(tasklist: &RwLock<Tasklist>,
                       ingestion_url: Arc<String>,
                       sender_channel: Sender<IngestionPacket>) -> () {

    let tasks = tasklist.read().unwrap().clone();
    info!("Tasklist akquired successfully.");

    for task in tasks.tasks.into_iter() {
        info!("Dispatching extractor Task for {}.", &task.url);
        let channel = sender_channel.clone();
        let url = ingestion_url.clone();
        match task.interface_type {
            InterfaceType::ShellyV1 => tokio::spawn(async move {collect::<ShellyV1Response>(task, &url, channel).await}),
            InterfaceType::ShellyV2 => tokio::spawn(async move {collect::<ShellyV2Response>(task, &url, channel).await}),
            InterfaceType::WeatherAPI => tokio::spawn(async move {collect::<WeatherResponse>(task, &url, channel).await}),
            //SensorType::Smartfox => collect::<SmartfoxResponse>(task)
        };
    }
}

pub async fn tasklist_observer(global_tasklist: &RwLock<Tasklist>, tasklist_url: &str) -> reqwest::Result<()> {
    let updated_tasks: Tasklist = reqwest::get(tasklist_url).await?.json::<Tasklist>().await?;

    let mut tasks = global_tasklist.write().unwrap();
    *tasks = updated_tasks;
    info!("Tasklist up to date.");
    Ok(())
}
