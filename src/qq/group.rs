use crate::qq::ai::get_ai_response;
use crate::qq::model::{Message, Payload};
use crate::qq::send::send_group_message;
use axum::Json;
use serde_json::{json, Value};

pub async fn handle_group_message(payload: Payload) -> Json<Value> {
    let d = payload.clone().d.unwrap();
    let mut content = d.content;
    if !content.starts_with("/") {
        content = get_ai_response(content).await;
    }
    let message = Message {
        content: Some(content),
        msg_type: 0,
        event_id: payload.id,
        msg_id: d.id,
        msg_seq: Some(1),
    };
    send_group_message(message, d.group_openid.unwrap()).await;
    Json(json!({}))
}