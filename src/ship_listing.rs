use serde_json::Value;

use crate::utils::*;
use crate::waypoint::*;

#[derive(Debug)]
pub struct ShipListing {
    symbol: String,
    price: i64,
}

impl ShipListing {
    pub fn from_json(value: &Value) -> ShipListing {
        ShipListing{
            symbol: as_string(&value["shipSymbol"]),
            price: value["price"].as_i64().unwrap(),
        }
    }
}