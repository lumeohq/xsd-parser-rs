use std::collections::HashMap;

use roxmltree::{Namespace, Node};

use crate::parser::attribute::parse_attribute;
use crate::parser::choice::parse_choice;
use crate::parser::complex_content::parse_complex_content;
use crate::parser::complex_type::parse_complex_type;
use crate::parser::constants::attribute;
use crate::parser::element::parse_element;
use crate::parser::sequence::parse_sequence;
use crate::parser::simple_content::parse_simple_content;
use crate::parser::simple_type::parse_simple_type;
use crate::parser::types::{File, Import, RsEntity, StructField};
use crate::parser::utils::{get_documentation, target_namespace};
use crate::parser::xsd_elements::{ElementType, XsdNode};

pub fn parse(text: &str) -> Result<File, ()> {
    let doc = roxmltree::Document::parse(&text).expect("Parse document error");
    let root = doc.root();

    let mut map = HashMap::new();

    let schema = root
        .children()
        .filter(|e| e.is_element())
        .last()
        .expect("Schema element is required");

    let rs_entity = parse_node(&schema, &root, None);

    if let RsEntity::File(rs_file) = rs_entity {
        for ty in &rs_file.types {
            if let RsEntity::Struct(st) = ty {
                map.extend(st.get_types_map());
            }
        }

        for ty in &rs_file.types {
            if let RsEntity::Struct(st) = ty {
                st.extend_base(&map);
            }
        }

        return Ok(rs_file);
    }

    Err(())
}

pub fn parse_node(node: &Node, parent: &Node, tn: Option<&Namespace>) -> RsEntity {
    use ElementType::*;

    match node.xsd_type() {
        Any => parse_any(node),
        AnyAttribute => parse_any_attribute(node),
        Attribute => parse_attribute(node, tn),
        Choice => parse_choice(node, tn),
        ComplexContent => parse_complex_content(node, tn),
        ComplexType => parse_complex_type(node, parent, tn),
        Element => parse_element(node, parent, tn),
        Import | Include => parse_import(node),
        Schema => parse_schema(node),
        Sequence => parse_sequence(node, parent, tn),
        SimpleContent => parse_simple_content(node, tn),
        SimpleType => parse_simple_type(node, parent, tn),

        _ => {
            unreachable!("{:?}", node);
        }
    }
}

pub fn parse_schema(schema: &Node) -> RsEntity {
    RsEntity::File(File {
        name: "".into(),
        namespace: None,
        types: schema
            .children()
            .filter(|n| n.is_element())
            .map(|node| parse_node(&node, schema, target_namespace(&schema)))
            .collect(),
    })
}

// Stubs
fn parse_import(node: &Node) -> RsEntity {
    RsEntity::Import(Import {
        name: node.attribute(attribute::NAMESPACE).unwrap_or("").into(),
        location: node
            .attribute(attribute::SCHEMA_LOCATION)
            .unwrap_or("")
            .into(),
    })
}

fn parse_any(node: &Node) -> RsEntity {
    RsEntity::StructField(StructField {
        name: "any".to_string(),
        type_name: "AnyElement".to_string(),
        macros: "//TODO: yaserde macro for any element\n//".to_string(),
        subtypes: vec![],
        comment: get_documentation(node),
    })
}

fn parse_any_attribute(node: &Node) -> RsEntity {
    RsEntity::StructField(StructField {
        name: "any_attribute".to_string(),
        type_name: "AnyAttribute".to_string(),
        macros: "//TODO: yaserde macro for any attribute\n//".to_string(),
        subtypes: vec![],
        comment: get_documentation(node),
    })
}
