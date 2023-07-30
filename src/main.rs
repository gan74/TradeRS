extern crate serde_json;

mod utils;
mod api;
mod agent;
mod waypoint;
mod contract;

use crate::utils::*;
use crate::agent::*;
use crate::api::*;
use crate::waypoint::*;
use crate::contract::*;

fn main() {
    let token = read_text_file("./token.txt").expect("Could not find token file");
    let api = Api::new(token);

    let contracts = api.contracts().unwrap();
    println!("{:?}", contracts);
}
