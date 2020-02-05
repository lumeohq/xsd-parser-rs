mod parser;

use std::fs;
use std::io::Read;

use crate::parser::parser::parse;


fn main() {
    let text = load_file("xsd/onvif.xsd");
    parse(text.as_str());

    println!("\n\n\n");
}

fn load_file(path: &str) -> String {
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

