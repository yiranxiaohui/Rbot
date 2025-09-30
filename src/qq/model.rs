use serde::{Deserialize, Serialize};
use crate::qq::file::File;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payload {
    pub id: Option<String>,
    pub op: Option<i16>,
    pub d: Option<D>,
    pub s: Option<i16>,
    pub t: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct D {
    pub id: Option<String>,
    pub author: Option<Author>,
    pub content: String,
    pub timestamp: Option<String>,
    pub group_openid: Option<String>,
    pub attachments: Option<String>,
    pub plain_token: Option<String>,
    pub event_ts: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Author {
    pub member_openid: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub content: Option<String>,
    pub msg_type: i16,
    pub event_id: Option<String>,
    pub msg_id: Option<String>,
    pub media: Option<File>,
    pub msg_seq: Option<i16>
}