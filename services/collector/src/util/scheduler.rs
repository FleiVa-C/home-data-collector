use crate::models::shelly_v1::ShellyV1Response;
use crate::models::shelly_v2::ShellyV2Response;
use crate::models::weather::WeatherResponse;
use crate::util::collector::extract;
use hdc_shared::config::config::*;
use serde::{Deserialize, Serialize};
use std::io;
use std::thread::spawn;

use super::collector::SensorType;

pub fn taskforce() -> io::Result<()> {
    let config = Config::new();

    let task1: Task = Task {
        url: config.shelly_v1_url,
        interface: SensorType::ShellyV1,
        signal_uuids: vec!["total_power".to_string()],
    };
    let task2: Task = Task {
        url: config.weather_url,
        interface: SensorType::WeatherAPI,
        signal_uuids: vec!["temp_current".to_string()],
    };

    let task3: Task = Task {
        url: config.shelly_v2_url,
        interface: SensorType::ShellyV2,
        signal_uuids: vec!["temp_100_uuid".to_string()],
    };

    let tasks = ConcurrentTasks {
        tasks: vec![task1, task2, task3],
    };
    let mut thread_handles = Vec::new();

    for task in tasks.tasks {
        thread_handles.push(spawn(move || match task.interface {
            SensorType::ShellyV1 => extract::<ShellyV1Response>(task),
            SensorType::ShellyV2 => extract::<ShellyV2Response>(task),
            SensorType::WeatherAPI => extract::<WeatherResponse>(task),
            //SensorType::Smartfox => extract::<SmartfoxResponse>(task)
        }));
    }
    for handle in thread_handles {
        let _ = handle.join().unwrap();
    }
    Ok(())
}

pub struct ConcurrentTasks {
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub url: String,
    pub interface: SensorType,
    pub signal_uuids: Vec<String>,
}
