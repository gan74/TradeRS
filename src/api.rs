use serde_json::Value;

use crate::request_manager::*;
use crate::agent::*;
use crate::waypoint::*;
use crate::contract::*;
use crate::ship_listing::*;

use std::cell::RefCell;
use std::rc::Rc;


pub struct Api {
    token: String,
    req_manager: Rc<RefCell<RequestManager>>,
}

#[derive(Debug)]
pub enum ApiError {
    ConnectionError,
    BadRequestError,
    JsonParseError,
    RateLimitExceeded,
}

impl Api {
    pub fn new(token: String, req_manager: Rc<RefCell<RequestManager>>) -> Api {
        Api {
            token: token,
            req_manager: req_manager,
        }
    }

    pub fn agent(&self) -> Result<Agent, ApiError> {
        let parsed = self.get_and_parse("https://api.spacetraders.io/v2/my/agent")?;
        Ok(Agent::from_json(&parsed["data"]))
    }

    pub fn waypoint(&self, waypoint: WaypointSymbol) -> Result<Waypoint, ApiError> {
        let url = format!("https://api.spacetraders.io/v2/systems/{}/waypoints/{}", waypoint.system(), waypoint.name());

        let parsed = self.get_and_parse(&url)?;
        Ok(Waypoint::from_json(&parsed["data"]))
    }

    pub fn contracts(&self) -> Result<Vec<Contract>, ApiError> {
        let parsed = self.get_and_parse("https://api.spacetraders.io/v2/my/contracts")?;
        
        match &parsed["data"] {
            Value::Array(arr) => Ok(arr.into_iter().map(|val| Contract::from_json(&val)).collect::<Vec<_>>()),
            _ => Ok(Vec::new()),
        }
    }

    pub fn accept(&self, contract: &Contract) -> Result<(), ApiError> {
        let url = format!("https://api.spacetraders.io/v2/my/contracts/{}/accept", contract.id);
        self.post_and_parse(&url)?;
        Ok(())
    }

    pub fn system_waypoints(&self, waypoint: WaypointSymbol) -> Result<Vec<Waypoint>, ApiError> {
        let url = format!("https://api.spacetraders.io/v2/systems/{}/waypoints", waypoint.system());

        let parsed = self.get_and_parse(&url)?;
        match &parsed["data"] {
            Value::Array(arr) => Ok(arr.into_iter().map(|val| Waypoint::from_json(&val)).collect::<Vec<_>>()),
            _ => Ok(Vec::new()),
        }
    }

    pub fn available_ships(&self, waypoint: WaypointSymbol) -> Result<Vec<ShipListing>, ApiError> {
        let url = format!("https://api.spacetraders.io/v2/systems/{}/waypoints/{}/shipyard", waypoint.system(), waypoint.name());

        let parsed = self.get_and_parse(&url)?;
        match &parsed["data"]["transactions"] {
            Value::Array(arr) => Ok(arr.into_iter().map(|val| ShipListing::from_json(&val)).collect::<Vec<_>>()),
            _ => Ok(Vec::new()),
        }
    }


    fn get_and_parse(&self, url: &str) -> Result<Value, ApiError> {
        let res = self.req_manager.borrow_mut().get(url, &self.token)?;
        let parsed = serde_json::from_str::<Value>(&res)?;
        check_is_error(&parsed)?;
        Ok(parsed)
    }

    fn post_and_parse(&self, url: &str) -> Result<Value, ApiError> {
        let res = self.req_manager.borrow_mut().post(url, &self.token)?;
        let parsed = serde_json::from_str::<Value>(&res)?;
        check_is_error(&parsed)?;
        Ok(parsed)
    }
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


impl From<serde_json::Error> for ApiError {
    fn from(_: serde_json::Error) -> Self {
        ApiError::JsonParseError
    }
}

impl From<RequestError> for ApiError {
    fn from(_: RequestError) -> Self {
        ApiError::ConnectionError
    }
}


