use serde_json::Value;

use crate::utils::*;
use crate::waypoint::*;

#[derive(Debug, Clone)]
pub struct Agent {
    id: String,
    symbol: String,
    hq: String,
    faction: String,

    creds: i64,
}

impl Agent {
    pub fn from_json(value: &Value) -> Self {
        Agent{
            id: as_string(&value["accountId"]),
            symbol: as_string(&value["symbol"]),
            hq: as_string(&value["headquarters"]),
            faction: as_string(&value["startingFaction"]),
            creds: value["credits"].as_i64().unwrap(),
        }
    }

    pub fn headquarters(&self) -> WaypointSymbol {
        WaypointSymbol::from_name(&self.hq)
    }

    pub fn credits(&self) -> i64 {
        self.creds
    }
}