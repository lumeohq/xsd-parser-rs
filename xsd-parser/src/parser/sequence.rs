use std::cell::RefCell;

use roxmltree::Node;

use crate::parser::node_parser::parse_node;
use crate::parser::types::{RsEntity, Struct, StructField, TypeModifier};
use crate::parser::utils::{enum_to_field, get_documentation, get_parent_name};
use crate::parser::xsd_elements::{ElementType, XsdNode};

pub fn parse_sequence(sequence: &Node, parent: &Node) -> RsEntity {
    let name = get_parent_name(sequence);
    RsEntity::Struct(Struct {
        name: name.into(),
        comment: get_documentation(parent),
        subtypes: vec![],
        fields: RefCell::new(elements_to_fields(sequence, name)),
        ..Default::default()
    })
}

fn elements_to_fields(sequence: &Node, parent_name: &str) -> Vec<StructField> {
    sequence
        .children()
        .filter(|n| n.is_element() && n.xsd_type() != ElementType::Annotation)
        .map(|n| match parse_node(&n, sequence) {
            RsEntity::StructField(mut sf) => {
                if sf.type_name.ends_with(parent_name) {
                    sf.type_modifiers.push(TypeModifier::Recursive)
                }
                sf
            }
            RsEntity::Enum(mut en) => {
                en.name = format!("{}Choice", parent_name);
                enum_to_field(en)
            }
            _ => unreachable!("\nError: {:?}\n{:?}", n, parse_node(&n, sequence)),
        })
        .collect()
}
