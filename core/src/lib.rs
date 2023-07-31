/*#![allow(dead_code)]
#![allow(unused_imports)] 

extern crate serde_json;

mod request_manager;
mod utils;
mod api;
mod agent;
mod waypoint;
mod contract;
mod ship;
mod outfitting;
mod ship_listing;

use crate::utils::*;
use crate::agent::*;
use crate::api::*;
use crate::waypoint::*;
use crate::contract::*;
use crate::ship::*;
use crate::outfitting::*;
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

    let all_ships = api.ships().unwrap();
    for ship in all_ships {
        if ship.is_drone() && ship.has_mounted(&MountType::MiningLaser) {
            println!("Mining drone: {:?}", ship);
            //api.move_to_orbit(&ship).unwrap();
            //api.extract(&ship).unwrap();
        }
    }

    println!("ok!");
}*/

#![allow(dead_code)]
#![allow(unused_imports)] 

extern crate serde_json;

pub mod request_manager;
pub mod utils;
pub mod api;
pub mod agent;
pub mod waypoint;
pub mod contract;
pub mod ship;
pub mod outfitting;
pub mod ship_listing;


