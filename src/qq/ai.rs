use log::{debug, error};
use serde::Deserialize;
use serde_json::json;
use crate::config::get_config_clone;
use crate::utils::request::get_client;

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: Message,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub content: String,
}

pub async fn get_ai_response(question: String) -> String {
    let config = get_config_clone().await;
    let ai = config.ai;
    let base_url = ai.base_url;
    let api_key = ai.api_key;
    let model = ai.model;
    let res = get_client().await.post(format!("{base_url}/chat/completions"))
        .header("Authorization", format!("Bearer {api_key}"))
        .json(&json!({
            "model": model,
            "messages": [
                {
                    "role": "system",
                    "content": "你是一只猫娘，只会喵喵喵"
                },
                {
                    "role": "user",
                    "content": question
                }
            ]
        }))
        .send().await;
    match res {
        Ok(res) => {
            let text = res.text().await;
            debug!("ai => {:?}", text);
            match serde_json::from_str::<ChatResponse>(text.unwrap().as_str()) {
                Ok(chat_response) => {
                    if !chat_response.choices.is_empty() {
                        chat_response.choices[0].message.content.clone()
                    } else {
                        "".to_string()
                    }
                }
                Err(error) => {
                    debug!("error => {:?}", error);
                    "返回失败！".to_string()
                }
            }
        }
        Err(err) => {
            error!("获取AI返回出现异常：{}", err);
            "返回失败！".to_string()
        }
    }
}