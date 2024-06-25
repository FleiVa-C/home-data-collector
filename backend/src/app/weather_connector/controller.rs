use super::error::Error;
use std::ops::Bound;

use crate::sdb::SDBRepository;
use hdc_shared::models::ingestion_container::*;
use hdc_shared::models::signal_data::*;
use hdc_shared::models::signal_meta::SignalMeta;
use hdc_shared::models::weather_adapter::WeatherAdapter;
use hdc_shared::models::weather_adapter::WeatherModel;

impl SDBRepository {
    pub async fn register_weather_connector(
        &self,
        interface: WeatherModel,
        instance: &str,
    ) -> Result<(), Error> {
        let mut existing = self
            .db
            .query(format!(
                "SELECT * FROM weather_connector WHERE url = '{}'",
                interface.url.clone()
            ))
            .await
            .map_err(|e| Error::Db {
                error: e,
                instance: instance.to_owned(),
            })?;
        let result: Vec<WeatherModel> = existing.take(0).map_err(|e| Error::Db {
            error: e,
            instance: instance.to_owned(),
        })?;
        match result.len() {
            0 => (),
            _ => {
                return Err(Error::WeatherConnectorAlreadyExists {
                    instance: instance.to_owned(),
                })
            }
        }
        let created: Option<WeatherModel> = self
            .db
            .create(("weather_connector", interface.uuid.clone().unwrap()))
            .content(&interface)
            .await
            .map_err(|e| Error::Db {
                error: e,
                instance: instance.to_owned(),
            })?;
        match created {
            Some(_) => Ok(()),
            None => Ok(()),
        }
    }
    pub async fn get_weather_meta(
        &self,
        ip: String,
        instance: &str,
    ) -> Result<WeatherModel, Error> {
        let mut meta = self
            .db
            .query(format!(
                "SELECT * FROM weather_connector WHERE url = '{}'",
                ip
            ))
            .await
            .map_err(|e| Error::Db {
                error: e,
                instance: instance.to_owned(),
            })?;

        let result: Vec<WeatherModel> = meta.take(0).map_err(|e| Error::Db {
            error: e,
            instance: instance.to_owned(),
        })?;
        Ok(result[0].clone())
    }
}
