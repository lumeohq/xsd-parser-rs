use std::collections::HashMap;

use roxmltree::Node;

use crate::parser::attribute::parse_attribute;
use crate::parser::choice::parse_choice;
use crate::parser::complex_content::parse_complex_content;
use crate::parser::complex_type::parse_complex_type;
use crate::parser::constants::attribute;
use crate::parser::element::parse_element;
use crate::parser::list::parse_list;
use crate::parser::sequence::parse_sequence;
use crate::parser::simple_content::parse_simple_content;
use crate::parser::simple_type::parse_simple_type;
use crate::parser::types::{File, Import, RsEntity, StructField, StructFieldSource, TypeModifier};
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

pub fn parse_node(node: &Node, parent: &Node) -> RsEntity {
    use ElementType::*;

    match node.xsd_type() {
        Any => parse_any(node),
        AnyAttribute => parse_any_attribute(node),
        Attribute => parse_attribute(node),
        Choice => parse_choice(node),
        ComplexContent => parse_complex_content(node),
        ComplexType => parse_complex_type(node, parent),
        Element => parse_element(node, parent),
        Import | Include => parse_import(node),
        List => parse_list(node),
        Sequence => parse_sequence(node, parent),
        SimpleContent => parse_simple_content(node),
        SimpleType => parse_simple_type(node, parent),

        _ => {
            unreachable!("{:?}", node);
        }
    }
}

pub fn parse_schema<'input>(schema: &Node<'_, 'input>) -> File<'input> {
    File {
        name: "".into(),
        namespace: None,
        target_ns: target_namespace(&schema).cloned(),
        types: schema
            .children()
            .filter(|n| n.is_element())
            .map(|node| parse_node(&node, schema))
            .collect(),
    }
}

// Stubs
fn parse_import(node: &Node) -> RsEntity {
    RsEntity::Import(Import {
        name: node.attribute(attribute::NAMESPACE).unwrap_or("").into(),
        location: node
            .attribute(attribute::SCHEMA_LOCATION)
            .unwrap_or("")
            .into(),
        comment: None,
    })
}

fn parse_any(node: &Node) -> RsEntity {
    RsEntity::StructField(StructField {
        name: "any".to_string(),
        type_name: "String".to_string(),
        comment: get_documentation(node),
        source: StructFieldSource::Element,
        type_modifiers: vec![TypeModifier::Option],
        ..Default::default()
    })
}

fn parse_any_attribute(node: &Node) -> RsEntity {
    RsEntity::StructField(StructField {
        name: "any_attribute".to_string(),
        type_name: "String".to_string(),
        comment: get_documentation(node),
        source: StructFieldSource::Attribute,
        type_modifiers: vec![TypeModifier::Option],
        ..Default::default()
    })
}
