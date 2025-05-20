use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
#[derive(Clone,Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AccountStatus {
    Active,
    Inactive,
    Suspended,
    Closed,
    Pending,
    Wait,
}

impl FromStr for AccountStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(AccountStatus::Active),
            "Inactive" => Ok(AccountStatus::Inactive),
            "Suspended" => Ok(AccountStatus::Suspended),
            "Closed" => Ok(AccountStatus::Closed),
            "Pending" => Ok(AccountStatus::Pending),
            "Wait" => Ok(AccountStatus::Wait),
            _ => Err(format!("Invalid AccountStatus: {}", s)),
        }
    }
}

impl fmt::Display for AccountStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}