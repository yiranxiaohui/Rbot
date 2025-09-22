use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QQ {
    pub app_id: u32,
    pub client_secret: String,
    pub interval: u64
}