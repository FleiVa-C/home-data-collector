use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SignalMetaQuery {
    name: Option<String>,
    uuid: Option<String>,
    interface_uuid: Option<String>,
}

impl SignalMetaQuery {
    pub fn build_sql_query(&self) -> String {
        let mut arg_count: u8 = 0;

        let name = match &self.name {
            Some(name) => {
                arg_count += 1;
                format!("name = '{}'", name)
            }
            None => "".to_owned(),
        };

        let interface_uuid = match &self.interface_uuid {
            Some(interface_uuid) => {
                if arg_count > 0 {
                    arg_count += 1;
                    format!("AND interface_uuid contains '{}'", interface_uuid)
                } else {
                    arg_count += 1;
                    format!("interface_uuid contains '{}'", interface_uuid)
                }
            }
            None => "".to_owned(),
        };
        if arg_count < 1 && self.uuid.is_none() {
            return format!("SELECT * FROM signal");
        }
        if self.uuid.is_some() {
            let uuid = &self.uuid.to_owned().unwrap();
            if arg_count > 0 {
                format!(
                    "SELECT * FROM signal:`{}` WHERE {}{}",
                    uuid, interface_uuid, name
                )
            } else {
                format!("SELECT * FROM signal:`{}`", uuid)
            }
        } else {
            format!(
                "SELECT * FROM signal WHERE {}{}",
                interface_uuid, name
            )
        }
    }
}
