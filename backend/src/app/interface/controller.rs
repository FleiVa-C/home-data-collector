use serde::{Serialize, Deserialize};
use surrealdb::sql::Value;

use hdc_shared::models::{interface::*, tasklist::*};
use crate::app::general::error::*;
use crate::sdb::SDBRepository;

impl SDBRepository {
    pub async fn register_interface(&self, interface: Interface) -> Result<(), DefaultError> {

        let created: Result<Option<Interface>, surrealdb::Error> = self
            .db
            .create(("interface", interface.base_url.clone()))
            .content(&interface)
            .await;
        match created {
            Ok(_) => Ok(()),
            Err(_) => Err(DefaultError::RegisterFailure(
                "Interface already exists.".to_string(),
            )),
        }
    }
    pub async fn get_interface(&self, interface: String) -> Option<Interface> {
        let response: Result<Option<Interface>, surrealdb::Error> = self
            .db
            .select(("interface", interface))
            .await;
        match response {
            Ok(response) => response,
            Err(_) => None,
        }
    }

    pub async fn get_all_interfaces(&self) -> Option<Vec<Interface>> {
        let response: Result<Vec<Interface>, surrealdb::Error> = self.db.select("interface").await;
        match response {
            Ok(response) => Some(response),
            Err(_) => None,
        }
    }
    
    pub async fn get_tasks(&self) -> Option<Vec<CollectorTask>>{
        let response: Result<Vec<Interface>, surrealdb::Error> = self.db.select("interface").await;
        match response {
            Ok(response) => {
                let tasklist = response
                    .into_iter()
                    .map(|entry| CollectorTask::from(entry))
                    .collect();
                Some(tasklist)},
            Err(_) => None
        }
    }
}
