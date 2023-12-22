use hdc_shared::models::interface::*;
use serde::{Serialize, Deserialize};

use crate::app::general::error::*;
use crate::sdb::SDBRepository;

impl SDBRepository {
    pub async fn register_interface(&self, interface: Interface<ShellyV1Adapter>) -> Result<(), DefaultError> {
        let created: Result<Option<Interface<ShellyV1Adapter>>, surrealdb::Error> = self
            .db
            .create(("interface", interface.base_url.clone()))
            .content(interface)
            .await;
        match created {
            Ok(_) => Ok(()),
            Err(_) => Err(DefaultError::RegisterFailure(
                "Interface already exists.".to_string(),
            )),
        }
    }
}
//    pub async fn get_interface(&self, interface: InterfaceIdentifier) -> Option<Interface> {
//        let response: Result<Option<Interface>, surrealdb::Error> = self
//            .db
//            .select(("interface", interface.get_global_id()))
//            .await;
//        match response {
//            Ok(response) => response,
//            Err(_) => None,
//        }
//    }
//
//    pub async fn get_all_interfaces(&self) -> Option<Vec<Interface>> {
//        let response: Result<Vec<Interface>, surrealdb::Error> = self.db.select("interface").await;
//        match response {
//            Ok(response) => Some(response),
//            Err(_) => None,
//        }
//    }
//}
