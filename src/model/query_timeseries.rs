use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct QueryTimeseriesData {
    pub signals: Vec<String>,
    pub time_from: i64,
    pub time_to: i64,
}
