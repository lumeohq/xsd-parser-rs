use roxmltree::Node;

use crate::parser::{
    types::{RsEntity, StructField, StructFieldSource, TypeModifier},
    utils::get_documentation,
};

pub fn parse_any(node: &Node) -> RsEntity {
    RsEntity::StructField(StructField {
        name: "any".to_string(),
        type_name: "String".to_string(),
        comment: get_documentation(node),
        source: StructFieldSource::Element,
        type_modifiers: vec![TypeModifier::Empty],
        ..Default::default()
    })
}
