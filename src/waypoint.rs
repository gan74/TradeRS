use serde_json::Value;

#[derive(Debug)]
pub struct Waypoint {
    symbol: String,
    wp_type: String,

    x: i64,
    y: i64,
}

impl Waypoint {
    pub fn from_json(value: &serde_json::Value) -> Result<Waypoint, ()> {
        match (&value["symbol"], &value["type"], &value["x"], &value["y"]) {
            (Value::String(sym), Value::String(wp_type), Value::Number(x), Value::Number(y)) => {
                Ok(Waypoint{
                    symbol: sym.clone(),
                    wp_type: wp_type.clone(),
                    x: x.as_i64().expect("X is not i64"),
                    y: y.as_i64().expect("Y is not i64"),
                })
            },

            _ => Err(()),
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
