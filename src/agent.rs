use serde_json::Value;

use crate::utils::*;

#[derive(Debug)]
pub struct Agent {
    id: String,
    symbol: String,
    pub headquarters: String,
    faction: String,

    credits: i64,
}

impl Agent {
    pub fn from_json(value: &Value) -> Agent {
        Agent{
            id: as_string(&value["accountId"]),
            symbol: as_string(&value["symbol"]),
            headquarters: as_string(&value["headquarters"]),
            faction: as_string(&value["startingFaction"]),
            credits: value["credits"].as_i64().unwrap(),
        }
    }
}