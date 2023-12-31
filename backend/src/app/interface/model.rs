use serde::{Deserialize, Serialize};

use hdc_shared::models::interface::InterfaceType;

#[derive(Serialize, Deserialize)]
pub struct InterfaceQuery {
    base_url: Option<String>,
    uuid: Option<String>,
    interface_type: Option<String>,
}

impl InterfaceQuery {
    pub fn build_sql_query(&self) -> String {
        let mut arg_count: u8 = 0;

        let url = match &self.base_url {
            Some(url) => {
                arg_count += 1;
                format!("base_url = '{}'", url)
            }
            None => "".to_owned(),
        };

        let uuid = match &self.uuid {
            Some(uuid) => {
                if arg_count > 0 {
                    arg_count += 1;
                    format!("AND uuid = '{}'", uuid)
                } else {
                    arg_count += 1;
                    format!("uuid = '{}'", uuid)
                }
            }
            None => "".to_owned(),
        };

        let interface = match &self.interface_type {
            Some(interface) => {
                if arg_count > 0 {
                    arg_count += 1;
                    format!("AND interface_type = '{}'", interface)
                } else {
                    arg_count += 1;
                    format!("interface_type = '{}'", interface)
                }
            }
            None => "".to_owned(),
        };
        format!(
            "SELECT * FROM interface WHERE {} {} {}",
            url, uuid, interface
        )
    }
}
