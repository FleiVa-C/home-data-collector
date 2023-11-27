use serde::{Serialize, Deserialize};
use uuid::Uuid;
use surrealdb::sql::{Object, Value, Thing};
use std::collections::BTreeMap;
use std::fmt::Debug;
use surrealdb::opt::RecordId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Signal {
    pub id: Option<RecordId>,
    pub uuid: String,
    pub name: String,
    pub uom: String,
    pub display_uom: String,
}

impl Signal {
    pub fn not_found(search: &String) -> Self {
        Signal {
        id: None,
        uuid: search.clone(),
        name: "not found".to_string(),
        uom: "not found".to_string(),
        display_uom: "not found".to_string()
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SignalIdentifier {
    pub signal_identifier: String
}

impl SignalIdentifier{
    pub fn new(identifier: String) -> SignalIdentifier{
        SignalIdentifier { signal_identifier: identifier }
    }
    pub fn get_global_id(&self) -> String {
        return format!("{}", self.signal_identifier);
    }
}

impl Signal {
    pub fn new(signal_uuid: String, signal_name: String, signal_uom: String,
               signal_display_uom: String) -> Signal {
        Signal{
            id: None,
            uuid: signal_uuid,
            name: signal_name,
            uom: signal_uom,
            display_uom: signal_display_uom
        }
    }

    pub fn get_global_id(&self) -> String {
        return format!("{}", self.uuid);
    }
}
