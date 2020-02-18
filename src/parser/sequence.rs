use std::cell::RefCell;

use roxmltree::{Namespace, Node};

use crate::parser::parser::parse_node;
use crate::parser::types::{Enum, RsEntity, Struct, StructField, StructFieldSource};
use crate::parser::utils::{
    get_documentation, get_parent_name, match_type,
};

pub fn parse_sequence(sequence: &Node, parent: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let name = get_parent_name(sequence);
    RsEntity::Struct(Struct {
        name: match_type(name, target_ns).into(),
        comment: get_documentation(parent),
        subtypes: vec![],
        fields: RefCell::new(elements_to_fields(sequence, name, target_ns)),
    })
}

fn elements_to_fields(
    sequence: &Node,
    parent_name: &str,
    target_ns: Option<&Namespace>,
) -> Vec<StructField> {
    sequence
        .children()
        .filter(|n| n.is_element())
        .map(|n| match parse_node(&n, sequence, target_ns) {
            RsEntity::StructField(sf) => sf,
            RsEntity::Enum(mut en) => {
                en.name = format!("{}Choice", match_type(parent_name, target_ns));
                enum_to_field(en, target_ns)
            }
            _ => unreachable!("\nError: {:?}\n{}", n, parse_node(&n, sequence, target_ns)),
        })
        .collect()
}

fn enum_to_field(en: Enum, target_ns: Option<&Namespace>) -> StructField {
    StructField {
        name: en.name.clone(),
        type_name: en.name.clone(),
        comment: None,
        subtypes: vec![RsEntity::Enum(en)],
        source: StructFieldSource::Element
    }
}
