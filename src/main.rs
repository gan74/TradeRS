extern crate serde_json;

mod utils;
mod api;
mod agent;
mod waypoint;

use crate::utils::*;
use crate::agent::*;
use crate::api::*;
use crate::waypoint::*;

fn main() {
    let token = read_text_file("./token.txt").expect("Could not find token file");
    let api = Api::new(token);

    let agent = api.agent().unwrap();
    println!("{:?}", agent);

    let hd = api.waypoint(&agent.headquarters).unwrap();
    println!("{:?}", hd);


}
