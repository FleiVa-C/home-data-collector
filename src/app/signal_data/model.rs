use serde::{Serialize, Deserialize};
use derive_more::Display;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct IngestionPacket {
    pub data : Vec<DataPoint>,
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
    pub success : Vec<DataPoint>,
    pub failed : Vec<DataPoint>,
    pub already_exists : Vec<DataPoint>
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
pub struct DataPoint {
    pub timestamp: i64,
    pub suuid: String,
    pub value: f64,
}

impl fmt::Display for DataPoint{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "timestamp: {}, uuid: {}, value: {}", self.timestamp, self.suuid, self.value);
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
pub struct DataValue {
    pub timestamp: i64,
    pub value: f64
}

#[derive(Serialize, Deserialize)]
pub struct SignalData {
    pub signal_uuid: String,
    pub signal_name: String,
    pub data: Vec<DataValue>,
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
