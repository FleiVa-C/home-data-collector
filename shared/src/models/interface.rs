use super::shelly_v1_adapter::ShellyV1Adapter;
use super::shelly_v2_adapter::ShellyV2Adapter;
use super::shelly_v1_adapter_light::ShellyV1AdapterLight;
use super::shelly_v2_adapter_light::ShellyV2AdapterLight;
use super::signal_meta::SignalMeta;
use super::weather_adapter::WeatherAdapter;
use super::tasklist::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::mem::discriminant;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "interface_type")]
pub enum InterfaceModel {
    ShellyV1(Interface<ShellyV1Adapter>),
    ShellyV2(Interface<ShellyV2Adapter>),
    WeatherIngestion(Interface<WeatherAdapter>),
}

impl InterfaceModel {
    pub fn get_url(&self) -> String {
        match self {
            Self::ShellyV1(value) => value.url.clone(),
            Self::ShellyV2(value) => value.url.clone(),
            Self::WeatherIngestion(value) => value.url.clone(),
        }
    }
    pub fn add_uuid(&mut self) {
        match self {
            Self::ShellyV1(value) => value.add_uuid(),
            Self::ShellyV2(value) => value.add_uuid(),
            Self::WeatherIngestion(value) => value.add_uuid(),
        }
    }
    pub fn get_signals(&self) -> Vec<SignalMeta> {
        match self {
            Self::ShellyV1(value) => value.signals.get_signals(),
            Self::ShellyV2(value) => value.signals.get_signals(),
            Self::WeatherIngestion(value) => value.signals.get_signals(),
        }
    }
    pub fn get_uuid(&self) -> Option<String> {
        match self {
            Self::ShellyV1(value) => value.uuid.clone(),
            Self::ShellyV2(value) => value.uuid.clone(),
            Self::WeatherIngestion(value) => value.uuid.clone(),
        }
    }
    pub fn check_update(&self, new_value: &Self) -> bool {
        if discriminant(self) == discriminant(new_value) {
            let existing_signals = match self {
                Self::ShellyV1(value) => value.signals.get_signals(),
                Self::ShellyV2(value) => value.signals.get_signals(),
                Self::WeatherIngestion(value) => value.signals.get_signals(),
            };
            let update_signals = match new_value {
                Self::ShellyV1(value) => value.signals.get_signals(),
                Self::ShellyV2(value) => value.signals.get_signals(),
                Self::WeatherIngestion(value) => value.signals.get_signals(),
            };
            let success: Option<()> = existing_signals
                .iter()
                .zip(update_signals.iter())
                .try_for_each(|(existing, update)| {
                    let existing_uuid = existing.get_uuid();
                    let update_uuid = update.get_uuid();
                    if existing_uuid == update_uuid {
                        Some(())
                    } else {
                        None
                    }
                });
            match success {
                Some(()) => return true,
                None => return false,
            };
        } else {
            return false;
        }
    }
    pub fn to_task(self) -> Option<CollectorTask> {
        let url: String = self.get_url();
        let adapter: Option<TaskType> = match self {
            InterfaceModel::ShellyV1(model) => {
                Some(TaskType::ShellyV1Task(ShellyV1AdapterLight::from(model.signals)))
            }
            InterfaceModel::ShellyV2(model) => {
                Some(TaskType::ShellyV2Task(ShellyV2AdapterLight::from(model.signals)))
            }
            InterfaceModel::WeatherIngestion(_)=> None
        };

        match adapter {
            None => return None,
            Some(value) => return 
                Some(CollectorTask {
                    url,
                    signals: value,
                })
        }
    }
}

pub trait IsAdapter {
    fn add_uuid(&mut self);
    fn get_signals(&self) -> Vec<SignalMeta>;
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Interface<T>
where
    T: IsAdapter,
{
    pub uuid: Option<String>,
    pub name: String,
    pub url: String,
    pub signals: T,
}

impl<T: IsAdapter> Interface<T> {
    pub fn add_uuid(&mut self) {
        self.uuid = Some(Uuid::new_v4().to_string());
        self.signals.add_uuid()
    }
    pub fn get_global_id(&self) -> &Option<String> {
        &self.uuid
    }
}
