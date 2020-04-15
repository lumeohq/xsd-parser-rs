extern crate clap;

use clap::{App, Arg};

use roxmltree::{Document, Node};
use std::fs;
use std::io::{prelude::*, Read};
use std::path::{Path, PathBuf};
use wsdl_parser::generator::generate;
use wsdl_parser::parser::definitions::Definitions;
use xsd_parser::generator::builder::GeneratorBuilder;
use xsd_parser::parser::schema::parse_schema;

fn main() {
    let matches = App::new("wsdl-parser")
        .about("An wsdl => rust code generator written in rust")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .takes_value(true)
                .help("Input .wsdl file"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("Output file"),
        )
        .get_matches();

    let input_path = matches.value_of("input").unwrap_or("input/wsdl");
    let input_path = Path::new(input_path);
    let output_path = matches.value_of("output");

    let md = fs::metadata(input_path).unwrap();
    if md.is_dir() {
        let output_path = Path::new(output_path.unwrap_or("output/wsdl"));
        process_dir(input_path, output_path)
    } else {
        process_single_file(input_path, output_path)
    }
    .map_err(|e| println!("Error: {}", e))
    .unwrap();
}

//TODO: Add a common mechanism for working with files
fn process_dir(input_path: &Path, output_path: &Path) -> Result<(), String> {
    if !output_path.exists() {
        fs::create_dir_all(output_path).map_err(|e| e.to_string())?;
    }
    for entry in fs::read_dir(input_path).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
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

fn process_single_file(input_path: &Path, output_path: Option<&str>) -> Result<(), String> {
    let text = load_file(input_path)?;
    let doc = Document::parse(text.as_str()).unwrap();
    let definitions = Definitions::new(&doc.root_element());
    let gen = GeneratorBuilder::default().build();
    let schemas = definitions
        .types()
        .iter()
        .flat_map(|t| t.schemas())
        .collect::<Vec<Node<'_, '_>>>();
    let mut code = schemas
        .iter()
        .map(|f| gen.generate_rs_file(&parse_schema(f)))
        .collect::<Vec<String>>();

    code.push(generate(&definitions));
    let code = code.join("");
    if let Some(output_filename) = output_path {
        write_to_file(output_filename, &code).map_err(|e| format!("Error writing file: {}", e))?;
    } else {
        println!("{}", code);
    }
    Ok(())
}

fn load_file(path: &Path) -> Result<String, String> {
    let mut file = fs::File::open(&path).map_err(|e| e.to_string())?;
    let mut text = String::new();
    file.read_to_string(&mut text).map_err(|e| e.to_string())?;
    Ok(text)
}

fn write_to_file(path: &str, text: &str) -> Result<(), String> {
    let mut file = fs::File::create(path).map_err(|e| e.to_string())?;
    file.write_all(text.as_bytes()).map_err(|e| e.to_string())
}
