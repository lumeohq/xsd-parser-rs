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
mod yaserde_generator;

use std::fs;
use std::io::Read;

use crate::generator::Generator;
use crate::parser::parse;
use crate::yaserde_generator::YaserdeGenerator;

fn main() {
    let text = load_file("xsd_external/xmlmime.xsd");
    //let text = load_file("xsd/onvif.xsd");
    if let Ok(f) = parse(text.as_str()) {
        let gen = YaserdeGenerator::new(&f);
        for ty in f.types {
            println!("{}", gen.gen_rs_entity(&ty));
        }
    }
}

fn load_file(path: &str) -> String {
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}
