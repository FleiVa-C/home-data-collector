use log::error;
use std::time::{Duration, SystemTime};
use std::sync::{RwLock, mpsc::Sender};

use super::collector::extract;
use crate::models::shelly_v1::ShellyV1Response;
use crate::models::shelly_v2::ShellyV2Response;
use crate::models::weather::WeatherResponse;
use hdc_shared::models::ingestion_container::IngestionPacket;
use hdc_shared::models::interface::InterfaceType;
use hdc_shared::models::tasklist::Tasklist;

pub async fn taskforce(tasklist: &RwLock<Tasklist>,
                       ingestion_url: &'static str,
                       sender_channel: Sender<IngestionPacket>) -> () {

    let tasks = tasklist.read().unwrap().clone();

    for task in tasks.tasks.into_iter() {
        let channel = sender_channel.clone();
        match task.interface_type {
            InterfaceType::ShellyV1 => tokio::spawn(async move {extract::<ShellyV1Response>(task, ingestion_url, channel).await}),
            InterfaceType::ShellyV2 => tokio::spawn(async move {extract::<ShellyV2Response>(task, ingestion_url, channel).await}),
            InterfaceType::WeatherAPI => tokio::spawn(async move {extract::<WeatherResponse>(task, ingestion_url, channel).await}),
            //SensorType::Smartfox => extract::<SmartfoxResponse>(task)
        };
    }
}

pub async fn tasklist_observer(global_tasklist: &RwLock<Tasklist>, tasklist_url: &str) -> reqwest::Result<()> {
    let updated_tasks: Tasklist = reqwest::get(tasklist_url).await?.json::<Tasklist>().await?;

    let mut tasks = global_tasklist.write().unwrap();
    *tasks = updated_tasks;
    println!("Updated Tasklist.");
    Ok(())
}
