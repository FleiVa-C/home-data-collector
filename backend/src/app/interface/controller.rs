use serde::{Deserialize, Serialize};
use surrealdb::{error::Api, sql::Value, Error};

use crate::app::general::error::BackendError;
use crate::sdb::SDBRepository;
use hdc_shared::models::signal_meta::SignalMeta;
use hdc_shared::models::{interface::*, tasklist::*};

use super::model::InterfaceQuery;

impl SDBRepository {
    pub async fn register_interface(&self, interface: Interface) -> Result<(), surrealdb::Error> {
        let mut existing = self
            .db
            .query(format!(
                "SELECT * FROM interface WHERE base_url = '{}'",
                interface.base_url
            ))
            .await?;
        let result: Vec<Interface> = existing.take(0)?;
        match result.len() {
            0 => (),
            _ => return Err(Error::Api(Api::Query("Already exists.".to_string()))),
        }

        let signals: Vec<SignalMeta> = interface.signals.get_signals();
        for signal in signals {
            let _: Option<SignalMeta> = self
                .db
                .create(("signal", signal.uuid.clone().unwrap()))
                .content(signal)
                .await?;
        }

        let created: Option<Interface> = self
            .db
            .create(("interface", interface.uuid.clone().unwrap()))
            .content(&interface)
            .await?;
        match created {
            Some(_) => Ok(()),
            None => Ok(()),
        }
    }

    pub async fn get_all_interfaces(&self) -> Result<Vec<Interface>, surrealdb::Error> {
        let response: Vec<Interface> = self.db.select("interface").await?;
        Ok(response)
    }

    pub async fn get_tasks(&self) -> Result<Vec<CollectorTask>, surrealdb::Error> {
        let response: Vec<Interface> = self.db.select("interface").await?;
        let tasklist = response
            .into_iter()
            .map(|entry| CollectorTask::from(entry))
            .collect();
        Ok(tasklist)
    }

    pub async fn query_interfaces(
        &self,
        interface_query: InterfaceQuery,
    ) -> Result<Vec<Interface>, surrealdb::Error> {
        let sql: String = interface_query.build_sql_query();
        let mut response = self.db.query(sql).await?;

        let result: Vec<Interface> = response.take(0)?;
        Ok(result)
    }
}
