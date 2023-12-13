use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IngestionPacket {
    pub data : Vec<Measurement>,
}

impl fmt::Display for IngestionPacket{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        for dp in &self.data{
            write!(f, "\t{}", dp)?;
        }
    Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Measurement {
    pub timestamp: i64,
    pub uuid: String,
    pub value: f64,
}

impl fmt::Display for Measurement{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "timestamp: {}, uuid: {}, value: {}", self.timestamp, self.uuid, self.value);
    Ok(())
    }
}
