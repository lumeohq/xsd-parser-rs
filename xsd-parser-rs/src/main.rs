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
use std::io::{self, prelude::*, Read};
use std::path::{Path, PathBuf};

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

    let input_path = matches.value_of("input").unwrap_or("xsd/onvif.xsd");
    let output_path = matches.value_of("output");

    let md = fs::metadata(input_path).unwrap();
    if md.is_file() {
        process_single_file(input_path, output_path);
    } else if md.is_dir() {
        let input_path = Path::new(input_path);
        let output_path = Path::new(output_path.unwrap_or("rs"));
        process_dir(input_path, output_path);
    }
}

fn process_single_file(input_path: &str, output_path: Option<&str>) {
    let text = match load_file(input_path) {
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
    if let Some(output_filename) = output_path {
        if let Err(e) = write_to_file(output_filename, &code) {
            println!("Error writing file: {}", e);
        }
    } else {
        println!("{}", code);
    }
}

fn process_dir(input_path: &Path, output_path: &Path) -> io::Result<()> {
    fs::create_dir(output_path);
    for entry in fs::read_dir(input_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            process_dir(&path, &output_path.join(path.file_name().unwrap()));
        } else {
            let output_file_path = PathBuf::new()
                .with_file_name(path.file_stem().unwrap().to_str().unwrap())
                .with_extension("rs");
            let output_file_path = output_path.join(output_file_path);
            println!("{}", output_file_path.to_str().unwrap());
            process_single_file(path.to_str().unwrap(), output_file_path.to_str());
        }
    }
    Ok(())
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
