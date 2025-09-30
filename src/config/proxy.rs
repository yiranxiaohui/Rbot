use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Proxy {
    pub address: String,
    pub port: u16,
}