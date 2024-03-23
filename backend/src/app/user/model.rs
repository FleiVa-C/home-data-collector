use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserQuery {
    pub name: Option<String>,
    pub firstname: Option<String>,
    pub uuid: Option<String>,
    pub email: Option<String>,
    pub is_admin: Option<bool>,
}

impl UserQuery {
    pub fn build_sql_query(&self) -> String {
        let mut arg_count: u8 = 0;

        let name = match &self.name {
            Some(name) => {
                arg_count += 1;
                format!("string::lowercase(name) contains '{}'", name.to_lowercase())
            }
            None => "".to_owned(),
        };

        let firstname = match &self.firstname {
            Some(firstname) => {
                if arg_count > 0 {
                    arg_count += 1;
                    format!(
                        " AND string::lowercase(firstname) contains '{}'",
                        firstname.to_lowercase()
                    )
                } else {
                    arg_count += 1;
                    format!(
                        "string::lowercase(firstname) contains '{}'",
                        firstname.to_lowercase()
                    )
                }
            }
            None => "".to_owned(),
        };

        let email = match &self.email {
            Some(email) => {
                if arg_count > 0 {
                    arg_count += 1;
                    format!(
                        " AND string::lowercase(email) contains '{}'",
                        email.to_lowercase()
                    )
                } else {
                    arg_count += 1;
                    format!(
                        "string::lowercase(email) contains '{}'",
                        email.to_lowercase()
                    )
                }
            }
            None => "".to_owned(),
        };

        let is_admin = match &self.is_admin {
            Some(is_admin) => {
                if arg_count > 0 {
                    arg_count += 1;
                    format!(" AND is_admin = {}", is_admin)
                } else {
                    arg_count += 1;
                    format!("is_admin = {}", is_admin)
                }
            }
            None => "".to_owned(),
        };
        if arg_count < 1 && self.uuid.is_none() {
            return format!("SELECT * FROM user");
        }
        if self.uuid.is_some() {
            let uuid = &self.uuid.to_owned().unwrap();
            if arg_count > 0 {
                format!(
                    "SELECT * FROM user:`{}` WHERE {}{}{}{}",
                    uuid, name, firstname, email, is_admin
                )
            } else {
                format!("SELECT * FROM user:`{}`", uuid)
            }
        } else {
            format!(
                "SELECT * FROM user WHERE {}{}{}{}",
                name, firstname, email, is_admin
            )
        }
    }
}
