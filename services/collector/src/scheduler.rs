use crate::models::shelly_v1::ShellyV1Response;
use crate::models::shelly_v2::ShellyV2Response;
use crate::models::weather::WeatherResponse;
use super::collector::extract;
use hdc_shared::models::tasklist::TaskList;
use hdc_shared::models::interface::InterfaceType;
use std::thread::spawn;

const URL:&str = "http://127.0.0.1:8080/v1/get_tasks";

pub fn taskforce() -> reqwest::Result<()> {
    let client = reqwest::blocking::Client::new();
    let tasks = client.get(URL).send()?.json::<TaskList>();

    let mut thread_handles = Vec::new();

    for task in tasks.unwrap().tasks.into_iter() {
        thread_handles.push(spawn(move || match task.interface_type {
            InterfaceType::ShellyV1 => extract::<ShellyV1Response>(task),
            InterfaceType::ShellyV2 => extract::<ShellyV2Response>(task),
            InterfaceType::WeatherAPI => extract::<WeatherResponse>(task),
            //SensorType::Smartfox => extract::<SmartfoxResponse>(task)
        }));
    }
    for handle in thread_handles {
        let _ = handle.join().unwrap();
    }
    Ok(())
}
