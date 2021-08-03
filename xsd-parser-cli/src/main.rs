use clap::{App, Arg};

use anyhow::Context;

use std::fs;
use std::io::{prelude::*, Read};
use std::path::{Path, PathBuf};

use std::fs::OpenOptions;
use xsd_parser::generator::builder::GeneratorBuilder;
use xsd_parser::parser::parse;

fn main() -> anyhow::Result<()> {
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

    let input_path = matches.value_of("input").unwrap_or("input/xsd");
    let input_path = Path::new(input_path);
    let output_path = matches.value_of("output");
    let md = fs::metadata(input_path).unwrap();
    if md.is_dir() {
        let output_path = Path::new(output_path.unwrap_or("output/rs"));
        process_dir(input_path, output_path)?;
    } else {
        process_single_file(input_path, output_path)?;
    }

    Ok(())
}

fn process_dir(input_path: &Path, output_path: &Path) -> anyhow::Result<()> {
    if !output_path.exists() {
        fs::create_dir_all(output_path)?;
    }
    for entry in fs::read_dir(input_path)? {
        let path = entry?.path();
        if path.is_dir() {
            process_dir(&path, &output_path.join(path.file_name().unwrap()))?;
        } else {
            let output_file_path = PathBuf::from(path.file_name().unwrap()).with_extension("rs");
            let output_file_path = output_path.join(output_file_path);
            process_single_file(&path, output_file_path.to_str())?;
        }
    }
    Ok(())
}

fn process_single_file(input_path: &Path, output_path: Option<&str>) -> anyhow::Result<()> {
    let text = load_file(input_path)?;
    let rs_file = parse(text.as_str()).map_err(|_| anyhow::anyhow!("Error parsing file"))?;
    let gen = GeneratorBuilder::default().build();
    let code = gen.generate_rs_file(&rs_file);
    if let Some(output_filename) = output_path {
        write_to_file(output_filename, &code).context("Error writing file")?;
    } else {
        println!("{}", code);
    }
    Ok(())
}

fn load_file(path: &Path) -> std::io::Result<String> {
    let mut file = fs::File::open(&path)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

fn write_to_file(path: &str, text: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;
    file.write_all(text.as_bytes())
}
