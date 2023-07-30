use serde_json::Value;

use std::fs::File;

use std::io::prelude::*;



pub fn as_string(val: &Value) -> String {
    val.as_str().unwrap().to_string()
}

pub fn read_text_file(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}