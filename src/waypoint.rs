use serde_json::Value;

use crate::utils::*;

#[derive(Debug)]
pub struct Waypoint {
    symbol: String,
    wp_type: String,

    x: i64,
    y: i64,
}

impl Waypoint {
    pub fn from_json(value: &Value) -> Waypoint {
        Waypoint{
            symbol: as_string(&value["symbol"]),
            wp_type: as_string(&value["type"]),
            x: value["x"].as_i64().unwrap(),
            y: value["y"].as_i64().unwrap(),
        }
    }
}


pub struct WaypointSymbol<'a> {
    pub waypoint: &'a str,
}

impl<'a> WaypointSymbol<'a> {
    pub fn from_waypoint(waypoint: &str) -> WaypointSymbol {
        WaypointSymbol {
            waypoint: waypoint,
        }
    }

    pub fn system(&self) -> &str {
        let wp_len = self.waypoint.rsplit('-').next().unwrap().len() + 1;
        &self.waypoint[..(self.waypoint.len() - wp_len)]
    }
}
