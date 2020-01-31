use roxmltree::{Node, Namespace};
use crate::generator2::types::{RsEntity, Struct, StructField, Enum};
use crate::generator2::utils::{match_type, struct_macro, get_documentation, get_parent_name, get_field_name, struct_field_macros};
use crate::generator2::generator::parse_node;

pub fn parse_sequence(sequence: &Node, parent: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let name = get_parent_name(sequence);
    RsEntity::Struct(
        Struct {
            name: match_type(name, target_ns).into(),
            macros: struct_macro(target_ns),
            comment: get_documentation(parent),
            subtypes: vec![],
            fields: elements_to_fields(sequence, name, target_ns)
        }
    )
}

fn elements_to_fields(sequence: &Node, parent_name: &str, target_ns: Option<&Namespace>) -> Vec<StructField> {
    sequence
    .children()
    .filter(|n| n.is_element())
    .map(|n| match parse_node(&n, sequence, target_ns){
        RsEntity::StructField(sf) => sf,
        RsEntity::Enum(mut en) => {
            en.name = match_type(parent_name, target_ns).into();
            return enum_to_field(en);
        },
        _ => unreachable!("\nElements of sequence may be only StructFields: {:?}\n{}", n, parse_node(&n, sequence, target_ns))
        }
    )
    .collect()
}

fn enum_to_field(en: Enum) -> StructField {
    StructField {
        name: format!("{}_choice", get_field_name(en.name.as_str())),
        type_name: format!("{}Choice", en.name),
        comment: None,
        macros: struct_field_macros(en.name.as_str()),
        subtypes: vec![RsEntity::Enum(en)]
    }
}