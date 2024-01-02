use super::collector::extract;
use crate::models::shelly_v1::ShellyV1Response;
use crate::models::shelly_v2::ShellyV2Response;
use crate::models::weather::WeatherResponse;
use hdc_shared::models::interface::InterfaceType;
use hdc_shared::models::tasklist::Tasklist;
use log::error;
use std::thread::spawn;
use std::sync::RwLock;

const URL: &str = "http://127.0.0.1:8080/v1/get_tasks";

pub fn taskforce(tasklist: &RwLock<Tasklist>) -> () {

    let tasks = tasklist.read().unwrap().clone();
    let mut thread_handles = Vec::new();

    for task in tasks.tasks.into_iter() {
        thread_handles.push(spawn(move || match task.interface_type {
            InterfaceType::ShellyV1 => extract::<ShellyV1Response>(task),
            InterfaceType::ShellyV2 => extract::<ShellyV2Response>(task),
            InterfaceType::WeatherAPI => extract::<WeatherResponse>(task),
            //SensorType::Smartfox => extract::<SmartfoxResponse>(task)
        }));
    }
    for handle in thread_handles {
        let status = handle.join().unwrap();
        match status {
            Ok(handle) => (),
            Err(e) => {
                error!("Something went wrong when trying to connect to {:}.", e.url().unwrap());
            }
        }

    }
}


pub fn tasklist_observer(global_tasklist: &RwLock<Tasklist>) -> reqwest::Result<()> {

    let client = reqwest::blocking::Client::new();
    let updated_tasks: Tasklist = client.get(URL).send()?.json::<Tasklist>().unwrap();

    let mut tasks = global_tasklist.write().unwrap();
    *tasks = updated_tasks;
    Ok(())
}
