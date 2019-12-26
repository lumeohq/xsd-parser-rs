use std::fs;
use std::io::{Read};

mod xsd2;
mod generator;
pub use generator::generator::Generator;
use crate::xsd2::utils::find_child;


fn main() {
    let text = load_file("xsd/onvif.xsd");
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

