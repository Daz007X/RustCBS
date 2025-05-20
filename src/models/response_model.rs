use serde::{Deserialize, Serialize};
use crate::models::{HeaderModel, RsBodyModel,PrSlkModel,pr_slk_model};
use chrono::DateTime;
use std::num::ParseIntError;
use regex::Regex;




#[derive(Debug, Serialize, Deserialize,Default)]
pub struct ResponseModel {
    #[serde(rename = "header")]
    pub header: HeaderModel,

    #[serde(rename = "rsBody")]
    pub rs_body: RsBodyModel,
}

impl ResponseModel {
    pub fn to_model(input_string: String) -> Result<ResponseModel, Box<dyn std::error::Error>> {
        let (input_result_string , prslk_vector)= Self::pullbucket(input_string).unwrap();
        let input = input_result_string.as_str();
        let mut header = HeaderModel::default();
        let mut rs_body = RsBodyModel::default();

        for part in input.split('|').filter(|s| !s.is_empty()) {
            let (section, kv) = part.split_once(',').ok_or("Invalid format: missing ','")?;
            let (key, value) = kv.split_once('=').ok_or("Invalid format: missing '='")?;

            match section {
                "header" => match key {
                    "Channel" => header.channel = value.to_string(),
                    "FuncNm" => header.func_nm = value.to_string(),
                    "MaxOccur" => header.max_occur = value.parse()?,  // แปลง String เป็น i32
                    "MsgId" => header.msg_id = value.to_string(),
                    "RqDt" => header.rq_dt = value.to_string(),
                    "RqUid" => header.rq_uid = value.to_string(),
                    "RsDt" => header.rs_dt = DateTime::parse_from_rfc3339(value)?,  // แปลง String เป็น DateTime
                    _ => (),
                },
                "rsBody" => match key {
                    "AcctId" => rs_body.acct_id = value.to_string(),
                    "ListPrSlk" => rs_body.list_pr_slk = prslk_vector.clone(),
                    "Period" => {
                        rs_body.period = value.parse().map_err(|e: ParseIntError| {
                            format!("Failed to parse 'Period': {}", e)
                        })?;
                    }
                    "PrName" => rs_body.pr_name = value.to_string(),
                    _ => (),
                },
                _ => (),
            }
        }

        Ok(ResponseModel { header, rs_body })
    }


    fn pullbucket(text: String) -> Option<(String,  Vec<PrSlkModel>)> {
        let re = Regex::new(r"\[(.*?)\]").unwrap();
        if let Some(caps) = re.captures(text.as_str()) {
            let bucket:Vec<PrSlkModel> = caps.get(1)
                            .unwrap()
                            .as_str()
                            .to_string()
                            .split(',')                         
                            .map(|s| s.trim_matches(|c| c == '{' || c == '}')) 
                            .map(|s| pr_slk_model::parse_prslk(s.to_string()).expect("ไม่สามารถ parse PrSlkModel ได้"))
                            .collect();             
            let replaced_text = re.replace(&text, "Pending_Vector").to_string();
            Some((replaced_text, bucket))
            } 
        else 
            {
            None
            }
    }
}


