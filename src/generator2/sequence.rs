use roxmltree::{Node, Namespace};
use crate::generator2::types::{RsEntity, Struct, StructField};
use crate::generator2::utils::{match_type, struct_macro, get_documentation, get_parent_name};
use crate::generator2::generator::parse_node;

pub fn parse_sequence(sequence: &Node, parent: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let name = get_parent_name(sequence);
    RsEntity::Struct(
        Struct {
            name: match_type(name, target_ns).into(),
            macros: struct_macro(target_ns),
            comment: get_documentation(parent),
            subtypes: vec![],
            fields: elements_to_fields(sequence, target_ns)
        }
    )
}

fn elements_to_fields(sequence: &Node, target_ns: Option<&Namespace>) -> Vec<StructField> {
    sequence
    .children()
    .filter(|n| n.is_element())
    .map(|n| match parse_node(&n, sequence, target_ns){
        RsEntity::StructField(sf) => sf,
        _ => unreachable!("Elements of sequence may be only StructFields: {:?}", n)
        }
    )
    .collect()
}