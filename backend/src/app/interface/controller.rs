use serde::{Deserialize, Serialize};
use surrealdb::{error::Api, sql::Value, Error};

use crate::app::general::error::BackendError;
use crate::sdb::SDBRepository;
use hdc_shared::models::signal_meta::SignalMeta;
use hdc_shared::models::{interface::*, tasklist::*};
use log::info;

use super::model::InterfaceQuery;

impl SDBRepository {
    pub async fn register_interface(
        &self,
        interface: InterfaceModel,
    ) -> Result<(), surrealdb::Error> {
        let mut existing = self
            .db
            .query(format!(
                "SELECT * FROM interface WHERE url = '{}'",
                interface.get_url()
            ))
            .await?;
        let result: Vec<InterfaceModel> = existing.take(0)?;
        match result.len() {
            0 => (),
            _ => return Err(Error::Api(Api::Query("Already exists.".to_string()))),
        }

        let signals: Vec<SignalMeta> = interface.get_signals();
        for signal in signals {
            let _: Option<SignalMeta> = self
                .db
                .create(("signal", signal.uuid.clone().unwrap()))
                .content(signal)
                .await?;
        }

        let created: Option<InterfaceModel> = self
            .db
            .create(("interface", interface.get_uuid().clone().unwrap()))
            .content(&interface)
            .await?;
        match created {
            Some(_) => Ok(()),
            None => Ok(()),
        }
    }

    pub async fn get_tasks(&self) -> Result<Vec<CollectorTask>, surrealdb::Error> {
        let response: Vec<InterfaceModel> = self.db.select("interface").await?;
        let tasklist = response
            .into_iter()
            .map(|entry| CollectorTask::from(entry))
            .collect();
        Ok(tasklist)
    }

    pub async fn query_interfaces(
        &self,
        interface_query: InterfaceQuery,
    ) -> Result<Vec<InterfaceModel>, surrealdb::Error> {
        let sql: String = interface_query.build_sql_query();
        let mut response = self.db.query(sql).await?;

        let result: Vec<InterfaceModel> = response.take(0)?;
        Ok(result)
    }
}
