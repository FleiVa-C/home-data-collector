use crate::models::shelly_v1::ShellyV1Response;
use crate::models::shelly_v2::ShellyV2Response;
use crate::models::weather::WeatherResponse;
use crate::util::collector::extract;
//use hdc_shared::config::config::*;
use hdc_shared::models::tasklist::*;
use hdc_shared::models::interface::*;
//use std::io;
use std::thread::spawn;

const URL:&str = "http://127.0.0.1:8080/v1/get_tasks";

pub fn taskforce() -> reqwest::Result<()> {
    //let config = Config::new();

    println!("getting task now");
    let client = reqwest::blocking::Client::new();
    let tasks = client.get(URL).send()?.json::<TaskList>();

    let mut thread_handles = Vec::new();

    for task in tasks.unwrap().tasks.into_iter() {
        println!("{:#?}", &task);
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
