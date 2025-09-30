use crate::features::ai::get_ai_response;
use crate::qq::model::{Message, Payload};
use crate::qq::send::send_group_message;
use axum::Json;
use serde_json::{json, Value};
use crate::features::news::get_news_response;
use crate::qq::file::{upload_group_file, FileType};

pub async fn handle_group_message(payload: Payload) -> Json<Value> {
    let d = payload.clone().d.unwrap();
    let mut content = d.clone().content;
    content = content.trim_start().to_string();
    if !content.starts_with("/") {
        content = get_ai_response(content).await;
    } else if content.starts_with("/每日新闻") {
        if let Ok(res) = get_news_response().await {
            if let Ok(file) = upload_group_file(d.clone().group_openid.unwrap(), res, FileType::Image).await {
                println!("{:?}", file);
                let message = Message {
                    content: Some(content),
                    msg_type: 7,
                    event_id: payload.id,
                    msg_id: d.id,
                    media: Some(file),
                    msg_seq: Some(1),
                };
                send_group_message(message, d.group_openid.unwrap()).await;
                return Json(json!({}));
            };
        }
    } else {
        content = "辉辉子不知道哦！".to_string();
    }
    let message = Message {
        content: Some(content),
        msg_type: 0,
        event_id: payload.id,
        msg_id: d.id,
        media: None,
        msg_seq: Some(1),
    };
    send_group_message(message, d.group_openid.unwrap()).await;
    Json(json!({}))
}

