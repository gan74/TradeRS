extern crate serde_json;

mod utils;
mod trader;

use crate::utils::*;
use crate::trader::*;

fn create_agent() -> Agent {
    let token = read_text_file("./token.json").expect("Could not find token file");
    Agent::from_json(&token).expect("Could not create agent from token")
}

fn main() {
    let agent = create_agent();

    println!("Hello, trader!");
}
