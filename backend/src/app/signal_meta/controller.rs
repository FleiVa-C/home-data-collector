use crate::app::signal_meta::error::SignalError;
use hdc_shared::models::signal_meta::SignalMeta;
use crate::sdb::SDBRepository;

use super::model::SignalMetaQuery;

impl SDBRepository {
    pub async fn register_signal(&self, signal: SignalMeta) -> Result<(), SignalError> {
        let created: Result<Option<SignalMeta>, surrealdb::Error> = self
            .db
            .create(("signal", signal.uuid.clone().unwrap()))
            .content(signal)
            .await;
        match created {
            Ok(_) => Ok(()),
            Err(_) => Err(SignalError::SignalRegisterFailure(
                "Signal Already Exists".to_string(),
            )),
        }
    }

    pub async fn get_all_signals(&self) -> Result<Vec<SignalMeta>, SignalError> {
        let response_data: Result<Vec<SignalMeta>, surrealdb::Error> = self.db.select("signal").await;
        match response_data {
            Ok(response_data) => Ok(response_data),
            Err(_) => Err(SignalError::SignalNotFound),
        }
    }
    pub async fn query_signal_meta(&self, query: SignalMetaQuery) -> Result<Vec<SignalMeta>, surrealdb::Error> {
        let sql: String = query.build_sql_query();
        let mut response = self.db
            .query(sql)
            .await?;

        let result: Vec<SignalMeta> = response.take(0)?;
        Ok(result)
    }
}
