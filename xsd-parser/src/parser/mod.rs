mod any;
mod any_attribute;
mod attribute;
mod attribute_group;
mod choice;
mod complex_content;
mod complex_type;
pub mod constants;
mod element;
mod extension;
mod import;
mod list;
mod node_parser;
mod restriction;
pub mod schema;
mod sequence;
mod simple_content;
mod simple_type;
mod tests;
pub mod types;
mod union;
mod utils;
pub mod xsd_elements;

use crate::parser::schema::parse_schema;
use crate::parser::types::{RsEntity, RsFile};
use std::collections::HashMap;

// FIXME: Actually pass up errors
#[allow(clippy::result_unit_err)]
pub fn parse(text: &str) -> Result<RsFile, ()> {
    let doc = roxmltree::Document::parse(text).expect("Parse document error");
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
    for ag in &schema_rs.attribute_groups {
        if let RsEntity::Struct(st) = ag {
            map.extend(st.get_types_map());
        }
    }
    for ty in &schema_rs.types {
        if let RsEntity::Struct(st) = ty {
            st.extend_base(&map);
            st.extend_attribute_group(&map);
        }
    }

    Ok(schema_rs)
}
