use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Features {
    pub ai: AI,
    pub news: News
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AI {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct News {
    pub url: String,
}