use serde_json::Value;
use curl::easy::{Easy, List};


use crate::agent::*;
use crate::waypoint::*;
use crate::contract::*;


pub struct Api {
    token: String,
}

#[derive(Debug)]
pub enum ApiError {
    ConnectionError,
    BadRequestError,
    EncodingError,
    JsonParseError,
    RateLimitExceeded,
}

impl Api {
    pub fn new(token: String) -> Api {
        Api {
            token: token
        }
    }

    pub fn agent(&self) -> Result<Agent, ApiError> {
        let parsed = curl_and_parse("https://api.spacetraders.io/v2/my/agent", &self.token)?;
        Ok(Agent::from_json(&parsed["data"]))
    }

    pub fn waypoint(&self, waypoint: &str) -> Result<Waypoint, ApiError> {
        let symbol = WaypointSymbol::from_waypoint(waypoint);
        let url = format!("https://api.spacetraders.io/v2/systems/{}/waypoints/{}", symbol.system(), symbol.waypoint);

        let parsed = curl_and_parse(&url, &self.token)?;
        Ok(Waypoint::from_json(&parsed["data"]))
    }

    pub fn contracts(&self) -> Result<Vec<Contract>, ApiError> {
        let parsed = curl_and_parse("https://api.spacetraders.io/v2/my/contracts", &self.token)?;
        
        match &parsed["data"] {
            Value::Array(arr) => Ok(arr.into_iter().map(|val| Contract::from_json(&val)).collect::<Vec<_>>()),
            _ => Ok(Vec::new()),
        }
    }
}











fn curl_and_parse(url: &str, token: &str) -> Result<Value, ApiError> {
    let res = curl(url, token)?;
    let parsed = serde_json::from_str::<Value>(&res)?;
    check_error(&parsed)?;
    Ok(parsed)
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