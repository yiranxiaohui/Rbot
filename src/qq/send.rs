use log::{debug, error};
use crate::qq::model::Message;
use crate::qq::token::{get_access_token_clone};
use crate::utils::request::get_client;

pub async fn send_group_message(message: Message, group_openid: String) {
    let url = format!("https://api.sgroup.qq.com/v2/groups/{group_openid}/messages");
    let access_token = get_access_token_clone().await;
    let res = get_client().await.post(url)
        .header("Authorization", format!("QQBot {access_token}"))
        .json(&message)
        .send().await;
    match res {
        Ok(res) => {
            let text = res.text().await.unwrap();
            debug!("res = {}", text);
        }
        Err(err) => {
            error!("发送消息出现异常：{}", err);
        }
    }
}