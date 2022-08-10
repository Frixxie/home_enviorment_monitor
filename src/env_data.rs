use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, sqlx::FromRow)]
pub struct EnvData {
    pub room: String,
    pub temp: f32,
    pub hum: f32,
}

impl EnvData {
    pub fn new(room: String, temp: f32, hum: f32) -> Self {
        Self { room, temp, hum }
    }
}

impl fmt::Display for EnvData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},{},{}",
            self.room,
            (self.temp as f32),
            (self.hum as f32),
        )
    }
}
