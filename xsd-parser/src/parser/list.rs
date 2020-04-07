use roxmltree::Node;

use crate::parser::constants::attribute;
use crate::parser::node_parser::parse_node;
use crate::parser::types::{RsEntity, TupleStruct, TypeModifier};
use crate::parser::utils::find_child;

pub fn parse_list(list: &Node) -> RsEntity {
    let mut result = match list.attribute(attribute::ITEM_TYPE) {
        Some(item_type) => TupleStruct {
            type_name: item_type.to_string(),
            ..Default::default()
        },
        None => {
            let nested_simple_type = find_child(list, "simpleType").expect(
                "itemType not allowed if the content contains a simpleType element. Otherwise, required."
            );

            match parse_node(&nested_simple_type, list) {
                RsEntity::Enum(en) => TupleStruct {
                    type_name: en.name.clone(),
                    subtypes: vec![RsEntity::Enum(en)],
                    ..Default::default()
                },
                RsEntity::TupleStruct(ts) => ts,
                _ => unreachable!(),
            }
        }
    };
    result.type_modifiers.push(TypeModifier::Array);
    RsEntity::TupleStruct(result)
}
