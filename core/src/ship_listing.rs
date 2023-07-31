use serde_json::Value;

use crate::utils::*;
use crate::waypoint::*;

#[derive(Debug)]
pub struct ShipListing {
    pub symbol: String,
    pub waypoint: String,
    pub price: i64,
}

impl ShipListing {
    pub fn from_json(value: &Value) -> Self {
        ShipListing{
            symbol: as_string(&value["shipSymbol"]),
            waypoint: as_string(&value["waypointSymbol"]),
            price: value["price"].as_i64().unwrap(),
        }
    }
}