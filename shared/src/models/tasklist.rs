use super::interface::{Interface, InterfaceModel, IsAdapter};
use super::shelly_v1_adapter_light::ShellyV1AdapterLight;
use super::shelly_v2_adapter_light::ShellyV2AdapterLight;
use super::weather_adapter_light::WeatherAdapterLight;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tasklist {
    pub tasks: Vec<CollectorTask>,
}

impl Tasklist {
    pub const fn new() -> Tasklist {
        Tasklist { tasks: Vec::new() }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollectorTask {
    pub url: String,
    pub signals: TaskType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaskType {
    ShellyV1Task(ShellyV1AdapterLight),
    ShellyV2Task(ShellyV2AdapterLight),
    WeatherTask(WeatherAdapterLight),
}

impl From<InterfaceModel> for CollectorTask {
    fn from(value: InterfaceModel) -> Self {
        let url: String = value.get_url();
        let adapter: TaskType = match value {
            InterfaceModel::ShellyV1(model) => {
                TaskType::ShellyV1Task(ShellyV1AdapterLight::from(model.signals))
            }
            InterfaceModel::ShellyV2(model) => {
                TaskType::ShellyV2Task(ShellyV2AdapterLight::from(model.signals))
            }
            InterfaceModel::WeatherAPI(model) => {
                TaskType::WeatherTask(WeatherAdapterLight::from(model.signals))
            }
        };

        CollectorTask {
            url,
            signals: adapter,
        }
    }
}
