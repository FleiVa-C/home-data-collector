use serde::{Serialize, Deserialize};
use super::interface::{Interface, InterfaceType, AdapterType};
use super::shelly_v1_adapter_light::ShellyV1AdapterLight;
use super::shelly_v2_adapter_light::ShellyV2AdapterLight;
use super::weather_adapter_light::WeatherAdapterLight;


#[derive(Serialize, Deserialize, Debug)]
pub struct TaskList {
    pub tasks: Vec<CollectorTask>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CollectorTask {
    pub url: String,
    pub interface_type: InterfaceType ,
    pub signals: TaskType,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TaskType{
    ShellyV1Task(ShellyV1AdapterLight),
    ShellyV2Task(ShellyV2AdapterLight),
    WeatherTask(WeatherAdapterLight)
}

impl From<Interface> for CollectorTask{
    fn from(value: Interface) -> Self {
        let adapter: TaskType = match value.signals{
            AdapterType::ShellyV1(adapter) => TaskType::ShellyV1Task(ShellyV1AdapterLight::from(adapter)),
            AdapterType::ShellyV2(adapter) => TaskType::ShellyV2Task(ShellyV2AdapterLight::from(adapter)),
            AdapterType::WeatherAPI(adapter) => TaskType::WeatherTask(WeatherAdapterLight::from(adapter)),
        };

        CollectorTask {
            url: value.base_url,
            interface_type: value.interface_type.unwrap(),
            signals: adapter }
    }
}
