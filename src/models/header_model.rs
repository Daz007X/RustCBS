use serde::{Deserialize, Serialize};
use chrono::{DateTime, FixedOffset};

#[derive(Debug, Serialize, Deserialize,Default)]
pub struct HeaderModel {
    #[serde(rename = "Channel")]
    pub channel: String,

    #[serde(rename = "FuncNm")]
    pub func_nm: String,

    #[serde(rename = "MaxOccur")]
    pub max_occur: i32,

    #[serde(rename = "MsgId")]
    pub msg_id: String,

    #[serde(rename = "RqDt")]
    pub rq_dt: String,

    #[serde(rename = "RqUid")]
    pub rq_uid: String,

    #[serde(rename = "RsDt")]
    pub rs_dt: DateTime<FixedOffset>,
}
