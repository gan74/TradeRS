use serde_json::Value;

use crate::utils::*;

#[derive(Debug)]
pub struct Contract {
    pub id: String,
    faction: String,
    contract_type: String,

    on_accepted: i64,
    on_fulfilled: i64,

    accepted: bool,
    fulfilled: bool
}

impl Contract {
    pub fn from_json(value: &Value) -> Contract {
        let payment = &value["terms"]["payment"];
        Contract{
            id: as_string(&value["id"]),
            faction: as_string(&value["factionSymbol"]),
            contract_type: as_string(&value["type"]),

            on_accepted: payment["onAccepted"].as_i64().unwrap(),
            on_fulfilled: payment["onFulfilled"].as_i64().unwrap(),

            accepted: value["accepted"].as_bool().unwrap(),
            fulfilled: value["fulfilled"].as_bool().unwrap(),
        }
    }
}