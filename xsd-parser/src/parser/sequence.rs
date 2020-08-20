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
    })
}

fn elements_to_fields(sequence: &Node, parent_name: &str) -> Vec<StructField> {
    sequence
        .children()
        .filter(|n| n.is_element() && n.xsd_type() != ElementType::Annotation)
        .flat_map(|n| {
            let parsed_node = parse_node(&n, sequence);
            match parsed_node {
                RsEntity::StructField(mut sf) => {
                    if sf.type_name.ends_with(parent_name) {
                        sf.type_modifiers.push(TypeModifier::Recursive)
                    }
                    vec![sf]
                }
                RsEntity::Enum(mut en) => {
                    en.name = format!("{}Choice", parent_name);
                    vec![enum_to_field(en)]
                }
                RsEntity::Struct(s) => s.fields.into_inner(),
                _ => unreachable!("\nError: {:?}\n{:?}", n, parsed_node),
            }
        })
        .collect()
}
