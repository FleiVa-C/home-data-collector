use surrealdb::error::Api;
use crate::sdb::SDBRepository;
use hdc_shared::models::signal_meta::SignalMeta;

use super::model::SignalMetaQuery;

impl SDBRepository {
    pub async fn register_signal(&self, signal: SignalMeta) -> Result<(), surrealdb::Error> {
        let created: Option<SignalMeta> = self
            .db
            .create(("signal", signal.uuid.clone().unwrap()))
            .content(signal)
            .await?;
        match created {
            Some(value) => Ok(()),
            None => Err(surrealdb::Error::Api(surrealdb::error::Api::Query("Signal already exists".to_string()))),
        }
    }

    pub async fn get_all_signals(&self) -> Result<Vec<SignalMeta>, surrealdb::Error> {
        let response: Vec<SignalMeta> = self
            .db
            .select("signal").await?;
        Ok(response)
    }
    pub async fn query_signal_meta(
        &self,
        query: SignalMetaQuery,
    ) -> Result<Vec<SignalMeta>, surrealdb::Error> {
        let sql: String = query.build_sql_query();
        let mut response = self.db.query(sql).await?;

        let result: Vec<SignalMeta> = response.take(0)?;
        Ok(result)
    }
}
