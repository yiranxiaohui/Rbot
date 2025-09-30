use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Proxy {
    pub enabled: bool,
    pub address: String,
    pub port: u16,
}