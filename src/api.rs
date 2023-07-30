use serde_json::Value;
use curl::easy::{Easy2, Handler, WriteError, List};

use crate::agent::*;
use crate::waypoint::*;
use crate::contract::*;
use crate::ship_listing::*;


pub struct Api {
    token: String,
}

#[derive(Debug)]
pub enum ApiError {
    ConnectionError,
    BadRequestError,
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
        let parsed = curl_get_and_parse("https://api.spacetraders.io/v2/my/agent", &self.token)?;
        Ok(Agent::from_json(&parsed["data"]))
    }

    pub fn waypoint(&self, waypoint: WaypointSymbol) -> Result<Waypoint, ApiError> {
        let url = format!("https://api.spacetraders.io/v2/systems/{}/waypoints/{}", waypoint.system(), waypoint.name());

        let parsed = curl_get_and_parse(&url, &self.token)?;
        Ok(Waypoint::from_json(&parsed["data"]))
    }

    pub fn contracts(&self) -> Result<Vec<Contract>, ApiError> {
        let parsed = curl_get_and_parse("https://api.spacetraders.io/v2/my/contracts", &self.token)?;
        
        match &parsed["data"] {
            Value::Array(arr) => Ok(arr.into_iter().map(|val| Contract::from_json(&val)).collect::<Vec<_>>()),
            _ => Ok(Vec::new()),
        }
    }

    pub fn accept(&self, contract: &Contract) -> Result<(), ApiError> {
        let url = format!("https://api.spacetraders.io/v2/my/contracts/{}/accept", contract.id);
        curl_post_and_parse(&url, &self.token)?;
        Ok(())
    }

    pub fn system_waypoints(&self, waypoint: WaypointSymbol) -> Result<Vec<Waypoint>, ApiError> {
        let url = format!("https://api.spacetraders.io/v2/systems/{}/waypoints", waypoint.system());

        let parsed = curl_get_and_parse(&url, &self.token)?;
        match &parsed["data"] {
            Value::Array(arr) => Ok(arr.into_iter().map(|val| Waypoint::from_json(&val)).collect::<Vec<_>>()),
            _ => Ok(Vec::new()),
        }
    }

    pub fn available_ships(&self, waypoint: WaypointSymbol) -> Result<Vec<ShipListing>, ApiError> {
        let url = format!("https://api.spacetraders.io/v2/systems/{}/waypoints/{}/shipyard", waypoint.system(), waypoint.name());

        let parsed = curl_get_and_parse(&url, &self.token)?;
        match &parsed["data"]["transactions"] {
            Value::Array(arr) => Ok(arr.into_iter().map(|val| ShipListing::from_json(&val)).collect::<Vec<_>>()),
            _ => Ok(Vec::new()),
        }
    }
}











fn curl_get_and_parse(url: &str, token: &str) -> Result<Value, ApiError> {
    let res = curl(url, token, RequestType::Get)?;
    let parsed = serde_json::from_str::<Value>(&res)?;
    check_is_error(&parsed)?;
    Ok(parsed)
}

fn curl_post_and_parse(url: &str, token: &str) -> Result<Value, ApiError> {
    let res = curl(url, token, RequestType::Post)?;
    let parsed = serde_json::from_str::<Value>(&res)?;
    check_is_error(&parsed)?;
    Ok(parsed)
}

fn check_is_error(value: &Value) -> Result<(), ApiError> {
    match &value["error"] {
        Value::Null => Ok(()),
        _ => {
            eprintln!("{value}");
            Err(ApiError::BadRequestError)
        },
    }
}

#[derive(PartialEq)]
enum RequestType {
    Get,
    Post,
}

fn curl(url: &str, token: &str, req_type: RequestType) -> Result<String, ApiError> {
    let mut easy = Easy2::new(Collector(Vec::new()));

    let mut list = List::new();
    list.append(&format!("Authorization: Bearer {token}"))?;
    easy.http_headers(list)?;

    match req_type {
        RequestType::Get => easy.get(true)?,
        RequestType::Post => easy.post(true)?,
    }

    easy.url(url)?;
    easy.perform()?;
 
    let contents = easy.get_ref();
    Ok(String::from_utf8_lossy(&contents.0).to_string())
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




struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}
