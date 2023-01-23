use serde::{Deserialize, Serialize};
use std::{
    fmt,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvDataEntry {
    pub timestamp: u64,
    pub room: String,
    pub temp: f32,
    pub hum: f32,
}

impl fmt::Display for EnvDataEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "timestamp: {}, room: {}, temp: {}, hum: {}",
            self.timestamp, self.room, self.temp, self.hum
        )
    }
}

impl From<EnvData> for EnvDataEntry {
    fn from(env_data: EnvData) -> Self {
        EnvDataEntry {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            room: env_data.room,
            temp: env_data.temp,
            hum: env_data.hum,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvData {
    room: String,
    temp: f32,
    hum: f32,
}

impl fmt::Display for EnvData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.room, self.temp, self.hum,)
    }
}
