use serde_json::Value;

use crate::utils::*;

use std::collections::HashSet;

#[derive(Debug)]
pub struct Waypoint {
    symbol: String,
    wp_type: String,

    traits: HashSet<String>,

    x: i64,
    y: i64,
}

impl Waypoint {
    pub fn from_json(value: &Value) -> Self {
        let traits = match &value["traits"] {
            Value::Array(arr) => arr.into_iter().map(|val| as_string(&val["symbol"])).collect::<HashSet<_>>(),
            _ => HashSet::new(),
        };

        Waypoint{
            symbol: as_string(&value["symbol"]),
            wp_type: as_string(&value["type"]),
            traits: traits,
            x: value["x"].as_i64().unwrap(),
            y: value["y"].as_i64().unwrap(),
        }
    }

    pub fn symbol(&self) -> WaypointSymbol { 
        WaypointSymbol::from_name(&self.symbol)
    }

    pub fn has_trait(&self, tr: &str) -> bool {
        self.traits.contains(tr)
    }
}



pub struct WaypointSymbol<'a> {
    waypoint: &'a str,
}

impl<'a> WaypointSymbol<'a> {
    pub fn from_name(waypoint: &str) -> WaypointSymbol {
        WaypointSymbol {
            waypoint: waypoint,
        }
    }

    pub fn name(&self) -> &str {
        self.waypoint
    }

    pub fn system(&self) -> &str {
        let wp_len = self.waypoint.rsplit('-').next().unwrap().len() + 1;
        &self.waypoint[..(self.waypoint.len() - wp_len)]
    }
}
