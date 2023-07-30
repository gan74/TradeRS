#![allow(dead_code)]
#[allow(unused_imports)] 

extern crate serde_json;

mod request_manager;
mod utils;
mod api;
mod agent;
mod waypoint;
mod contract;
mod ship_listing;

use crate::utils::*;
use crate::agent::*;
use crate::api::*;
use crate::waypoint::*;
use crate::contract::*;
use crate::ship_listing::*;
use crate::request_manager::*;

use std::fs::File;
use std::cell::RefCell;
use std::rc::Rc;


fn create_request_manager() -> Rc<RefCell<RequestManager>> {
    let file = File::create("api.log").expect("Unable to create log file");
    Rc::new(RefCell::new(RequestManager::new(file)))
}

fn main() {
    let token = read_text_file("./token.txt").expect("Could not find token file");
    let api = Api::new(token, create_request_manager());

    let agent = api.agent().unwrap();
    println!("{:?}", agent);

    let system_waypoints = api.system_waypoints(agent.headquarters()).unwrap();
    for wp in system_waypoints {
        if wp.has_trait("SHIPYARD") {
            let sp = api.available_ships(wp.symbol()).unwrap();
            println!("{:?}", sp);
        }
    }


}
