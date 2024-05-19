use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InterfaceQuery {
    pub url: Option<String>,
    pub uuid: Option<String>,
    pub interface_type: Option<String>,
    pub name: Option<String>,
}

impl InterfaceQuery {
    pub fn build_sql_query(&self) -> String {
        let mut arg_count: u8 = 0;

        let url = match &self.url {
            Some(url) => {
                arg_count += 1;
                format!("url = '{}'", url)
            }
            None => "".to_owned(),
        };

        let interface = match &self.interface_type {
            Some(interface) => {
                if arg_count > 0 {
                    arg_count += 1;
                    format!(
                        " AND string::lowercase(interface_type) contains '{}'",
                        interface.to_lowercase()
                    )
                } else {
                    arg_count += 1;
                    format!(
                        "string::lowercase(interface_type) contains '{}'",
                        interface.to_lowercase()
                    )
                }
            }
            None => "".to_owned(),
        };

        let name = match &self.name {
            Some(name) => {
                if arg_count > 0 {
                    arg_count += 1;
                    format!(
                        " AND string::lowercase(name) contains '{}'",
                        name.to_lowercase()
                    )
                } else {
                    arg_count += 1;
                    format!("string::lowercase(name) contains '{}'", name.to_lowercase())
                }
            }
            None => "".to_owned(),
        };
        if arg_count < 1 && self.uuid.is_none() {
            return format!("SELECT * FROM interface");
        }
        if self.uuid.is_some() {
            let uuid = &self.uuid.to_owned().unwrap();
            if arg_count > 0 {
                format!(
                    "SELECT * FROM interface:`{}` WHERE {}{}{}",
                    uuid, url, interface, name
                )
            } else {
                format!("SELECT * FROM interface:`{}`", uuid)
            }
        } else {
            format!("SELECT * FROM interface WHERE {}{}{}", url, interface, name)
        }
    }
}
