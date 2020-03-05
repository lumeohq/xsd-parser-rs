mod any;
mod any_attribute;
mod attribute;
mod choice;
mod complex_content;
mod complex_type;
mod element;
mod import;
mod list;
mod node_parser;
mod schema;
mod sequence;
mod simple_content;
mod simple_type;
mod tests;
mod utils;
pub mod constants;
pub mod types;
pub mod xsd_elements;

use crate::parser::schema::parse_schema;
use crate::parser::types::{File, RsEntity};
use std::collections::HashMap;

pub fn parse(text: &str) -> Result<File, ()> {
    let doc = roxmltree::Document::parse(&text).expect("Parse document error");
    let root = doc.root();

    let mut map = HashMap::new();

    let schema = root
        .children()
        .filter(|e| e.is_element())
        .last()
        .expect("Schema element is required");

    let schema_rs = parse_schema(&schema);
    for ty in &schema_rs.types {
        if let RsEntity::Struct(st) = ty {
            map.extend(st.get_types_map());
        }
    }
    for ty in &schema_rs.types {
        if let RsEntity::Struct(st) = ty {
            st.extend_base(&map);
        }
    }

    Ok(schema_rs)
}
