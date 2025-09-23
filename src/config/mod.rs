mod qq;
mod ai;

use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Arc;
use tokio::sync::{Mutex, OnceCell};
use crate::config::ai::AI;
use crate::config::qq::QQ;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Config {
    pub address: String,
    pub port: u16,
    pub log_level: String,
    pub qq: QQ,
    pub ai: AI
}

pub static CONFIG: OnceCell<Arc<Mutex<Config>>> = OnceCell::const_new();

pub async fn _get_config() -> Arc<Mutex<Config>> {
    CONFIG.get_or_init(init_config).await.clone()
}

pub async fn get_config_clone() -> Config {
    let config_arc = CONFIG.get_or_init(init_config).await;
    config_arc.clone().lock().await.clone()
}

async fn init_config() -> Arc<Mutex<Config>> {
    let string = fs::read_to_string("config.toml").unwrap();
    let config = match toml::from_str(&string) {
        Ok(config) => config,
        Err(e) => {
            println!("解析配置文件出现异常：e => {}", e);
            println!("正在退出程序！");
            std::process::exit(1);
        }
    };
    Arc::new(Mutex::new(config))
}