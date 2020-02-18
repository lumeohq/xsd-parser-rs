use std::cell::RefCell;

use roxmltree::Node;

use crate::parser::parser::parse_node;
use crate::parser::types::{Enum, RsEntity, Struct, StructField, StructFieldSource};
use crate::parser::utils::{get_documentation, get_parent_name};

pub fn parse_sequence(sequence: &Node, parent: &Node) -> RsEntity {
    let name = get_parent_name(sequence);
    RsEntity::Struct(Struct {
        name: name.into(),
        comment: get_documentation(parent),
        subtypes: vec![],
        fields: RefCell::new(elements_to_fields(sequence, name)),
    })
}

fn elements_to_fields(sequence: &Node, parent_name: &str) -> Vec<StructField> {
    sequence
        .children()
        .filter(|n| n.is_element())
        .map(|n| match parse_node(&n, sequence) {
            RsEntity::StructField(sf) => sf,
            RsEntity::Enum(mut en) => {
                en.name = format!("{}Choice", parent_name);
                enum_to_field(en)
            }
            _ => unreachable!("\nError: {:?}\n{}", n, parse_node(&n, sequence)),
        })
        .collect()
}

fn enum_to_field(en: Enum) -> StructField {
    StructField {
        name: en.name.clone(),
        type_name: en.name.clone(),
        subtypes: vec![RsEntity::Enum(en)],
        source: StructFieldSource::Element,
        ..Default::default()
    }
}
