use crate::sdb::SDBRepository;
use hdc_shared::models::user::User;
use surrealdb::error::Api;

use super::model::UserQuery;

impl SDBRepository {
    pub async fn register_user(&self, user: User) -> Result<(), surrealdb::Error> {
        let created: Option<User> = self
            .db
            .create(("user", user.uuid.clone().unwrap()))
            .content(user)
            .await?;
        match created {
            Some(value) => Ok(()),
            None => Err(surrealdb::Error::Api(surrealdb::error::Api::Query(
                "Signal already exists".to_string(),
            ))),
        }
    }

    pub async fn query_user(&self, query: UserQuery) -> Result<Vec<User>, surrealdb::Error> {
        let sql: String = query.build_sql_query();
        let mut response = self.db.query(sql).await?;

        let result: Vec<User> = response.take(0)?;
        Ok(result)
    }
}
