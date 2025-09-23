use axum::Json;
use ed25519_dalek::{Signer, SigningKey};
use serde_json::{json, Value};
use crate::config::get_config_clone;
use crate::qq::model::Payload;

pub fn signature(secret: &str, msg: &str) -> String {
    // 创建种子数据
    let mut seed = secret.to_string();
    while seed.len() < 32 {
        seed = seed.repeat(2);
    }
    seed.truncate(32);

    // 将种子转换为字节数组
    let seed_bytes = seed.as_bytes();

    let seed: [u8; 32] = seed_bytes[..32].try_into().unwrap();
    let private_key = SigningKey::from_bytes(&seed);
    let signature = private_key.sign(msg.as_bytes());
    // 转换为十六进制字符串
    hex::encode(signature.to_bytes())
}

pub async fn handle_signature(payload: Payload) -> Json<Value> {
    let msg = format!("{}{}", payload.clone().d.unwrap().event_ts.unwrap(), payload.clone().d.unwrap().plain_token.unwrap());
    let config = get_config_clone().await;
    let sig = signature(config.qq.client_secret.as_str(), msg.as_str());
    Json(json!({
        "plain_token": payload.d.unwrap().plain_token.unwrap(),
        "signature": sig,
    }))
}