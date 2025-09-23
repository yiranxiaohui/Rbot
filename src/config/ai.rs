use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AI {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
}