use serde_json::Value;
use curl::easy::{Easy, List};


use crate::agent::*;
use crate::waypoint::*;


pub struct Api {
    token: String,
}

#[derive(Debug)]
pub enum ApiError {
    ConnectionError,
    BadRequestError,
    EncodingError,
    JsonParseError,
    ResponseParseError,
    RateLimitExceeded,
}

impl Api {
    pub fn new(token: String) -> Api {
        Api {
            token: token
        }
    }

    pub fn agent(&self) -> Result<Agent, ApiError> {
        let res = curl("https://api.spacetraders.io/v2/my/agent", &self.token)?;
        let parsed = serde_json::from_str::<Value>(&res)?;
        check_error(&parsed)?;
        if let Ok(agent) = Agent::from_json(&parsed["data"]) {
            Ok(agent)
        } else {
            Err(ApiError::ResponseParseError)
        }
    }

    pub fn waypoint(&self, waypoint: &str) -> Result<Waypoint, ApiError> {
        let symbol = WaypointSymbol::from_waypoint(waypoint);
        let url = format!("https://api.spacetraders.io/v2/systems/{}/waypoints/{}", symbol.system(), symbol.waypoint);
        let res = curl(&url, &self.token)?;
        let parsed = serde_json::from_str::<Value>(&res)?;
        check_error(&parsed)?;
        if let Ok(wp) = Waypoint::from_json(&parsed["data"]) {
            Ok(wp)
        } else {
            Err(ApiError::ResponseParseError)
        }
    }
}











fn curl(url: &str, token: &str) -> Result<String, ApiError> {
    let mut buffer = Vec::new();

    {
        let mut easy = Easy::new();
        easy.url(url)?;

        let mut list = List::new();
        list.append(&format!("Authorization: Bearer {token}"))?;
        easy.http_headers(list)?;

        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            buffer.extend_from_slice(data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }

    if let Ok(res) = String::from_utf8(buffer) {
        Ok(res)
    } else {
        Err(ApiError::EncodingError)
    }
}

fn check_error(value: &Value) -> Result<(), ApiError> {
    match &value["error"] {
        Value::Null => Ok(()),
        _ => Err(ApiError::BadRequestError),
    }
}

impl From<curl::Error> for ApiError {
    fn from(_: curl::Error) -> Self {
        ApiError::ConnectionError
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(_: serde_json::Error) -> Self {
        ApiError::JsonParseError
    }
}