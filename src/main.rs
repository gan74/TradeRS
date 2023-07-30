#![allow(dead_code)]
#[allow(unused_imports)] 

extern crate serde_json;

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

fn main() {
    let token = read_text_file("./token.txt").expect("Could not find token file");
    let api = Api::new(token);

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
