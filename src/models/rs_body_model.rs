use serde::{Deserialize, Serialize};
use crate::models::{PrSlkModel};

#[derive(Debug, Serialize, Deserialize,Default)]
pub struct RsBodyModel {
    #[serde(rename = "AcctId")]
    pub acct_id: String,

    #[serde(rename = "ListPrSlk")]
    pub list_pr_slk: Vec<PrSlkModel>,

    #[serde(rename = "Period")]
    pub period: i32,

    #[serde(rename = "PrName")]
    pub pr_name: String,
}
