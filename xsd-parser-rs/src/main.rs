#[cfg(test)]
#[macro_use]
extern crate log;
#[cfg(test)]
#[macro_use]
extern crate yaserde_derive;

mod generator;
mod parser;
#[cfg(test)]
mod tests;

use std::fs;
use std::io::Read;

use crate::generator::builder::GeneratorBuilder;
use crate::parser::parse;

fn main() {
    let text = load_file("xsd_external/ws-discovery.xsd");
    //let text = load_file("xsd/onvif.xsd");
    if let Ok(f) = parse(text.as_str()) {
        let gen = GeneratorBuilder::default().build();
        println!("{}", gen.generate_rs_file(&f));
    }
}

fn load_file(path: &str) -> String {
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}
