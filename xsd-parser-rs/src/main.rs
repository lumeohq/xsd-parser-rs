extern crate clap;
use clap::{App, Arg};

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
use std::io::prelude::*;
use std::io::Read;

use crate::generator::builder::GeneratorBuilder;
use crate::parser::parse;

fn main() {
    let matches = App::new("xsd-parser-rs")
        .about("An xsd/wsdl => rust code generator written in rust")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .takes_value(true)
                .help("Input .xsd file"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("Output file"),
        )
        .get_matches();

    let input_filename = matches.value_of("input").unwrap_or("xsd/onvif.xsd");
    let output = matches.value_of("output");

    let text = match load_file(input_filename) {
        Ok(t) => t,
        Err(e) => {
            println!("Error loading file: {}", e);
            return;
        }
    };

    let rs_file = match parse(text.as_str()) {
        Ok(f) => f,
        _ => {
            println!("Error parsing file");
            return;
        }
    };

    let gen = GeneratorBuilder::default().build();
    let code = gen.generate_rs_file(&rs_file);
    if let Some(output_filename) = output {
        if let Err(e) = write_to_file(output_filename, &code) {
            println!("Error writing file: {}", e);
        }
    } else {
        println!("{}", code);
    }
}

fn load_file(path: &str) -> Result<String, String> {
    let mut file = fs::File::open(&path).map_err(|e| e.to_string())?;
    let mut text = String::new();
    file.read_to_string(&mut text).map_err(|e| e.to_string())?;
    Ok(text)
}

fn write_to_file(path: &str, text: &str) -> Result<(), String> {
    let mut file = fs::File::create(path).map_err(|e| e.to_string())?;
    file.write_all(text.as_bytes()).map_err(|e| e.to_string())
}
