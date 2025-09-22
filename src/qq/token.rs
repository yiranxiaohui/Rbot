use std::sync::{LazyLock};
use std::time::Duration;
use log::{debug, error};
use serde::de;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::{Mutex};
use tokio_cron_scheduler::{Job, JobScheduler};
use crate::config::{get_config_clone};

#[derive(Deserialize, Serialize)]
#[derive(Debug)]
pub struct Token {
    pub access_token: String,
    #[serde(deserialize_with = "string_to_u16")]
    pub expires_in: u16,
}

pub static TOKEN: LazyLock<Mutex<Option<Token>>> = LazyLock::new(|| {
    Mutex::new(None)
});

fn string_to_u16<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<u16>().map_err(de::Error::custom)
}

pub async fn get_app_access_token() -> Token {
    let config = get_config_clone().await;
    let client = reqwest::Client::new();
    let json = json!({
        "appId": config.qq.app_id.to_string(),
        "clientSecret": config.qq.client_secret
    });
    println!("{:?}", json);
    let res = client.post("https://bots.qq.com/app/getAppAccessToken")
        .json(&json)
    .send().await.unwrap().text().await.unwrap();
    debug!("{:?}", res);
    let token: Token = serde_json::from_str(&res).unwrap();
    token
}

async fn set_app_access_token() {
    let token = get_app_access_token().await;
    let mut guard = TOKEN.lock().await;
    *guard = Some(token);  // 直接解引用赋值
    debug!("access_token 设置成功");
}

pub async fn init() {
    set_app_access_token().await;
    let interval = get_config_clone().await.qq.interval;
    let duration = Duration::from_secs(interval);
    let scheduler = match JobScheduler::new().await {
        Ok(scheduler) => scheduler,
        Err(err) => {
            error!("创建定时器失败：{}", err);
            return;
        }
    };
    let job = match Job::new_repeated_async(duration, move |_context, _job| {
        Box::pin(async move {
            set_app_access_token().await;
            debug!("set_app_access_token方法执行成功！");
        })
    }) {
        Ok(job) => job,
        Err(e) => {
            error!("创建任务失败：{}", e);
            return;
        }
    };
    if let Err(e) = scheduler.add(job).await {
        error!("添加定时任务失败：{}", e);
    }
    if let Err(e) = scheduler.start().await {
        error!("启动定时器失败：{}", e);
    }
}