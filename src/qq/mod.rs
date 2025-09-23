use axum::body::{to_bytes, Body};
use axum::http::Request;
use axum::Json;
use log::{error, info};
use serde_json::{json, Value};
use crate::qq::group::handle_group_message;
use crate::qq::model::Payload;
use crate::qq::signature::{handle_signature};

mod token;
mod signature;
mod model;
mod group;
mod send;

pub async fn init() {
    token::init().await;
}

pub async fn webhook(request: Request<Body>) -> Json<Value> {
    let payload = get_payload(request).await.map_err(|e| {
        return Json(json!({}));
    }).unwrap();
    let op = payload.op.unwrap();
    match op {
        13 => handle_signature(payload).await,
        0 => handle_group_message(payload).await,
        _ => {
            info!("payload => {:?}", payload);
            Json(json!({}))
        }
    }
}

async fn get_payload(request: Request<Body>) -> Result<Payload, String> {
    // 处理 body_bytes 的 Result
    let body_bytes = match to_bytes(request.into_body(), usize::MAX).await {
        Ok(bytes) => bytes,
        Err(e) => {
            println!("Error reading body: {}", e);
            return Err(String::new());
        }
    };
    match serde_json::from_slice::<Payload>(body_bytes.to_vec().as_slice()) {
        Ok(payload) => Ok(payload),
        Err(e) => {
            error!("反序列化错误：e => {}", e);
            Err(String::new())
        }
    }
}