use serde::{Serialize, Deserialize};
use strum_macros::Display;
use surrealdb::sql::{Object, Value, Thing};
use surrealdb::opt::RecordId;
use std::fmt::Debug;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct IngestionPacket {
    pub data : Vec<DataPoint>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataPoint {
    pub timestamp: i64,
    pub suuid: String,
    pub value: f64,
}

impl fmt::Display for IngestionPacket{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        for dp in &self.data{
            write!(f, "\t{}", dp)?;
        }
    Ok(())
    }
}


impl DataPoint {
    fn new(ts: i64,signal_uuid: String, measured_value: f64) -> Self {
        DataPoint{
            timestamp: ts,
            suuid: signal_uuid,
            value: measured_value
        }
    }
}
impl fmt::Display for DataPoint{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "timestamp: {}, uuid: {}, value: {}", self.timestamp, self.suuid, self.value);
    Ok(())
    }
}

impl IngestionPacket {
    pub fn new(datapoints: Vec<DataPoint>) -> Self {
        IngestionPacket{
            data: datapoints
        }
    }
}
