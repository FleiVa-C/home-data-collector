use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct SignalMetaQuery{
    name: Option<String>,
    uuid: Option<String>,
    interface_uuid: Option<String>
}

impl SignalMetaQuery{
    pub fn build_sql_query(&self) -> String {
        let mut arg_count: u8 = 0;

        let name = match &self.name{
           Some(name) => {
               arg_count += 1;
               format!("name = '{}'", name)
           },
           None => "".to_owned()
        };

        let uuid = match &self.uuid{
           Some(uuid) => {
               if arg_count > 0 {
                   arg_count += 1;
                   format!("AND uuid = '{}'", uuid)
                }else{
                    arg_count += 1;
                    format!("uuid = '{}'", uuid)
                }
           },
           None => "".to_owned()
        };

        let interface_uuid = match &self.interface_uuid{
           Some(interface_uuid) => {
               if arg_count > 0 {
                   arg_count += 1;
                   format!("AND interface_uuid = '{}'", interface_uuid)
                }else{
                    arg_count += 1;
                    format!("interface_uuid = '{}'", interface_uuid)
                }
           },
           None => "".to_owned()
        };
        format!("SELECT * FROM signal WHERE {} {} {}", name, uuid, interface_uuid)
    }
}

