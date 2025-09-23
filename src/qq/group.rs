use axum::Json;
use serde_json::{json, Value};
use crate::qq::model::{Message, Payload, D};
use crate::qq::send::send_group_message;

pub async fn handle_group_message(payload: Payload) -> Json<Value> {
    let d = payload.clone().d.unwrap();
    let content = d.content;
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