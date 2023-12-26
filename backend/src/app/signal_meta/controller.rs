use crate::app::signal_meta::error::SignalError;
use crate::app::signal_meta::model::{Signal, SignalIdentifier};
use crate::sdb::SDBRepository;

impl SDBRepository {
    pub async fn register_signal(&self, signal: Signal) -> Result<(), SignalError> {
        let created: Result<Option<Signal>, surrealdb::Error> = self
            .db
            .create(("signal", signal.get_global_id()))
            .content(signal)
            .await;
        match created {
            Ok(_) => Ok(()),
            Err(_) => Err(SignalError::SignalRegisterFailure(
                "Signal Already Exists".to_string(),
            )),
        }
    }

    pub async fn get_signal(&self, signal: SignalIdentifier) -> Option<Signal> {
        let response: Result<Option<Signal>, surrealdb::Error> =
            self.db.select(("signal", signal.get_global_id())).await;
        match response {
            Ok(output) => output,
            Err(_) => None,
        }
    }

    pub async fn get_all_signals(&self) -> Result<Vec<Signal>, SignalError> {
        let response_data: Result<Vec<Signal>, surrealdb::Error> = self.db.select("signal").await;
        match response_data {
            Ok(response_data) => Ok(response_data),
            Err(_) => Err(SignalError::SignalNotFound),
        }
    }
}
