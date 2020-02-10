mod parser;

use std::fs;
use std::io::Read;

use crate::parser::parser::parse;

fn main() {
    let text = load_file("xsd/onvif.xsd");
    if let Ok(f) = parse(text.as_str()) { println!("{}", f) }
}

fn load_file(path: &str) -> String {
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}
