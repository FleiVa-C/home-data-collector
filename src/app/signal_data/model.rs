use serde::{Serialize, Deserialize};
use derive_more::Display;
use std::fmt;

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


#[derive(Serialize, Deserialize, Debug)]
pub struct MultiStatusData {
    pub success : Vec<Measurement>,
    pub failed : Vec<Measurement>,
    pub already_exists : Vec<Measurement>
}

impl fmt::Display for MultiStatusData{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        for dp in &self.success{
            write!(f, "\t{}", dp)?;
        }
        for dp in &self.failed{
            write!(f, "\t{}", dp)?;
        }
        for dp in &self.already_exists{
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

pub enum IngestionResponse{
    Success,
    MultiStatus(MultiStatusData)
}

#[derive(Serialize, Deserialize)]
pub struct QueryTimeseriesData {
    pub signals: Vec<String>,
    pub time_from: i64,
    pub time_to: i64,
}

#[derive(Serialize, Deserialize)]
pub struct DataPoint {
    pub timestamp: i64,
    pub value: f64
}

impl From<&Measurement> for DataPoint {
    fn from(ms: &Measurement) -> Self{
        DataPoint{
            timestamp: ms.timestamp,
            value: ms.value
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SignalData {
    pub signal_uuid: String,
    pub signal_name: String,
    pub data: Vec<DataPoint>,
    pub uom: String,
    pub display_uom: String
}

#[derive(Serialize, Deserialize)]
pub struct QueryResult {
    pub data: Vec<SignalData>
}

pub enum QueryResponse{
    Success(QueryResult),
    Failed
}
