use clap::Parser;

use anyhow::Context;

use roxmltree::{Document, Node};
use std::fs;
use std::io::{prelude::*, Read};
use std::path::{Path, PathBuf};
use wsdl_parser::generator::generate;
use wsdl_parser::parser::definitions::Definitions;
use xsd_parser::generator::builder::GeneratorBuilder;
use xsd_parser::parser::schema::parse_schema;

#[derive(Parser)]
#[clap(name = env!("CARGO_PKG_NAME"))]
#[clap(version = env!("CARGO_PKG_VERSION"))]
#[clap(about = env!("CARGO_PKG_DESCRIPTION"))]
struct Opt {
    /// Input .wsdl file
    #[clap(long, short)]
    input: Option<PathBuf>,

    /// Output file
    #[clap(long, short)]
    output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let opt: Opt = Opt::parse();

    let input_path = opt.input.unwrap_or_else(|| PathBuf::from("input/wsdl"));
    let md = fs::metadata(&input_path).unwrap();
    if md.is_dir() {
        let output_path = opt
            .output
            .unwrap_or_else(|| PathBuf::from("output/wsdl-rs"));
        process_dir(&input_path, &output_path)?;
    } else {
        process_single_file(&input_path, opt.output.as_deref())?;
    }

    Ok(())
}

//TODO: Add a common mechanism for working with files
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
            process_single_file(&path, Some(&output_file_path))?;
        }
    }
    Ok(())
}

fn process_single_file(input_path: &Path, output_path: Option<&Path>) -> anyhow::Result<()> {
    let text = load_file(input_path)?;
    let doc = Document::parse(text.as_str()).context("Failed to parse input document")?;
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
        write_to_file(output_filename, &code).context("Error writing file")?;
    } else {
        println!("{}", code);
    }
    Ok(())
}

fn load_file(path: &Path) -> std::io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

fn write_to_file(path: &Path, text: &str) -> std::io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(text.as_bytes())?;

    Ok(())
}
