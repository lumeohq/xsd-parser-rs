use std::fs;
use std::io::{Read};

mod xsd;
mod generator;
mod generator2;

pub use generator::generator::Generator;
use crate::xsd::utils::find_child;

use crate::generator2::generator::parse;


fn main() {
    let text = load_file("xsd/types.xsd");
    parse(text.as_str());

    println!("\n\n\n");

    let doc = match roxmltree::Document::parse(&text) {
        Ok(doc) => doc,
        Err(e) => {
            println!("Error: {}.", e);
            return;
        },
    };
    let root = doc.root();
    let schema = find_child(&root, "schema").expect("All xsd need schema element");
    let generator = Generator::new(schema);
    generator.print();
}

fn load_file(path: &str) -> String {
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

