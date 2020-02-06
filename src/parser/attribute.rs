use roxmltree::{Node, Namespace};
use crate::parser::types::{RsEntity, StructField};
use crate::parser::utils::{struct_field_macros, get_documentation, get_field_name, match_type};

pub fn parse_attribute(node: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let name =  node
        .attribute("name")
        .or(node.attribute("ref"))
        .expect("All attributes have name or ref in Onvif");

    let matched_type = match_type(
        node.attribute("type")
            .or(node.attribute("ref"))
            .unwrap_or("()"),
        target_ns,
    );

    let type_name = match node.attribute("use") {
        Some(u) => {match u {
            "optional" => format!("Option<{}>", matched_type),
            "prohibited" => "()".to_string(), // TODO: maybe Empty<T> or remove this field
            "required" => matched_type.to_string(),
            _ => unreachable!(
                "If 'use' specified, this attribute must have one of the following values [optional, prohibited, required]"
            )
        }},
        None => format!("Option<{}>", matched_type)
    };

    RsEntity::StructField(
        StructField {
            macros: struct_field_macros(name),
            type_name,
            comment: get_documentation(node),
            subtypes: vec![],
            name:  get_field_name(name),
        }
    )
}