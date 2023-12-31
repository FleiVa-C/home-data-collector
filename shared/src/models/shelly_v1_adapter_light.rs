use super::shelly_v1_adapter::ShellyV1Adapter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ShellyV1AdapterLight {
    pub emeter_1: String,
    pub emeter_2: String,
    pub emeter_3: String,
    pub emeter_4: String,
}

impl From<ShellyV1Adapter> for ShellyV1AdapterLight {
    fn from(value: ShellyV1Adapter) -> Self {
        ShellyV1AdapterLight {
            emeter_1: String::from(value.emeter_1),
            emeter_2: String::from(value.emeter_2),
            emeter_3: String::from(value.emeter_3),
            emeter_4: String::from(value.emeter_4),
        }
    }
}

impl ShellyV1AdapterLight {
    pub fn iter(&self) -> ShellyV1Iterator<'_> {
        ShellyV1Iterator {
            inner: self,
            index: 0,
        }
    }
}

pub struct ShellyV1Iterator<'a> {
    inner: &'a ShellyV1AdapterLight,
    index: u8,
}

impl<'a> Iterator for ShellyV1Iterator<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => &self.inner.emeter_1,
            1 => &self.inner.emeter_2,
            2 => &self.inner.emeter_3,
            3 => &self.inner.emeter_4,
            _ => return None,
        };
        self.index += 1;
        Some(ret)
    }
}
