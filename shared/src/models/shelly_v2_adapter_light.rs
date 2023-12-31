use super::shelly_v2_adapter::ShellyV2Adapter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ShellyV2AdapterLight {
    pub temp_100: String,
    pub temp_101: String,
    pub temp_102: String,
    pub temp_103: String,
}

impl From<ShellyV2Adapter> for ShellyV2AdapterLight {
    fn from(value: ShellyV2Adapter) -> ShellyV2AdapterLight {
        ShellyV2AdapterLight {
            temp_100: String::from(value.temp_100),
            temp_101: String::from(value.temp_101),
            temp_102: String::from(value.temp_102),
            temp_103: String::from(value.temp_103),
        }
    }
}

impl ShellyV2AdapterLight {
    pub fn iter(&self) -> ShellyV2Iterator<'_> {
        ShellyV2Iterator {
            inner: self,
            index: 0,
        }
    }
}

pub struct ShellyV2Iterator<'a> {
    inner: &'a ShellyV2AdapterLight,
    index: u8,
}

impl<'a> Iterator for ShellyV2Iterator<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => &self.inner.temp_100,
            1 => &self.inner.temp_101,
            2 => &self.inner.temp_102,
            3 => &self.inner.temp_103,
            _ => return None,
        };
        self.index += 1;
        Some(ret)
    }
}
