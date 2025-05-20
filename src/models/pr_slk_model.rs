use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use crate::enums::account_status::AccountStatus;
use std::str::FromStr;

#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct PrSlkModel {
    #[serde(rename = "Ccid")]
    pub ccid: i32,

    #[serde(rename = "Lotl")]
    pub lotl: i64,

    #[serde(rename = "Loth")]
    pub loth: i64,

    #[serde(rename = "Unit")]
    pub unit: i32,

    #[serde(rename = "Bal")]
    pub bal: f64,

    #[serde(rename = "Stat")]
    pub stat: i32,

    #[serde(rename = "Statd")]
    pub statd: AccountStatus,

    #[serde(rename = "Grp")]
    pub grp: String,

    #[serde(rename = "OpenDt")]
    pub open_dt: NaiveDate,

    #[serde(rename = "PrPaidLife")]
    pub pr_paid_life: i32,
}


pub fn parse_prslk(input: String) -> Result<PrSlkModel, Box<dyn std::error::Error>> {
    let trimmed = input.trim_start_matches('{').trim_end_matches('}');
    let mut model = PrSlkModel {
        ccid: 0, // ค่าเริ่มต้น
        lotl: 0,
        loth: 0,
        unit: 0,
        bal: 0.0,
        stat: 0,
        statd: AccountStatus::Active, // ต้องมี Default impl
        grp: String::new(),
        open_dt: NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(), 
        pr_paid_life: 0,
    };

    for pair in trimmed.split('|') {
        if let Some((key, value)) = pair.split_once('=') {
            match key {
                "Ccid" => model.ccid = value.parse()?,
                "Lotl" => model.lotl = value.parse()?,
                "Loth" => model.loth = value.parse()?,
                "Unit" => model.unit = value.parse()?,
                "Bal" => model.bal = value.parse()?,
                "Stat" => model.stat = value.parse()?,
                "Statd" => model.statd = AccountStatus::from_str(value)?, // ต้องมี from_str impl
                "Grp" => model.grp = value.to_string(),
                "OpenDt" => model.open_dt = NaiveDate::parse_from_str(value, "%Y-%m-%d")?,
                "PrPaidLife" => model.pr_paid_life = value.parse()?,
                _ => eprintln!("Unknown field: {}", key),
            }
        }
    }

    Ok(model)
}
