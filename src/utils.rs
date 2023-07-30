use std::fs::File;

use std::io::prelude::*;

use std::io;


pub fn read_text_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}