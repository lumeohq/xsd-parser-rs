use std::borrow::Cow;
use std::cell::RefCell;

use roxmltree::Node;
use roxmltree::Namespace;

use crate::parser::constants::attribute;
use crate::parser::parser::parse_node;
use crate::parser::types::{Alias, RsEntity, Struct, StructField, EnumCase};
use crate::parser::utils::{get_documentation, match_type, struct_macro, get_field_name, get_type_name, yaserde_for_element};
use crate::parser::xsd_elements::{ElementType, XsdNode, min_occurs, max_occurs, MaxOccurs};

pub fn parse_element(node: &Node, parent: &Node, target_ns: Option<&roxmltree::Namespace>) -> RsEntity {
    match parent.xsd_type() {
        ElementType::Schema => parse_global_element(node, target_ns),
        ElementType::Sequence => parse_field_of_sequence(node, target_ns),
        ElementType::Choice => parse_case_of_choice(node, target_ns),
        _ => element_default(node, target_ns)
    }
}

fn element_default(node: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let ty = node.attr_type().unwrap_or("UNSUPPORTED");
    RsEntity::Alias(
        Alias {
            name: match_type("UNSUPPORTED", target_ns).into(),
            original: match_type(ty, target_ns).into(),
            comment: get_documentation(node),
            subtypes: vec![],
        }
    )
}

fn parse_case_of_choice(element: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    if element.has_attribute(attribute::REF) {
        let ref_attr = element.attr_ref().unwrap();
        let name = ref_attr.split(":").last().unwrap();
        let type_name = element_type(element,match_type(ref_attr, target_ns));

        return RsEntity::EnumCase(
            EnumCase{
                name: name.to_string(),
                value: String::default(),
                type_name: Some(type_name.into()),
                comment: get_documentation(element),
                //subtypes: vec![]
            }
        );
    }

    let name = element.attr_name().unwrap_or("UNSUPPORTED_ELEMENT_NAME");

    if element.has_attribute(attribute::TYPE) {
        let type_name = element_type(
            element,
            match_type(element.attr_type().unwrap(), target_ns)
        );
        return RsEntity::EnumCase(
            EnumCase{
                name: name.to_string(),
                value: String::default(),
                type_name: Some(type_name.into()),
                comment: get_documentation(element),
                //subtypes: vec![]
            }
        );
    }

    let type_name = element_type(
        element,
        match_type(
            element.attr_type().unwrap_or("UNSUPPORTED_TYPE_OF_ELEMENT"),
            target_ns
        )
    );


    RsEntity::EnumCase(
        EnumCase{
            name: get_type_name(name),
            value: String::default(),
            type_name: Some(type_name),
            comment: get_documentation(element),
            //subtypes: vec![]
        }
    )
}

fn parse_field_of_sequence(node: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let name = node.attr_name().unwrap_or("UNSUPPORTED_ELEMENT_NAME");

    let type_name = element_type(
        node,
        match_type(
            node.attr_type().unwrap_or("UNSUPPORTED_TYPE_OF_ELEMENT"),
            target_ns
        )
    );

    RsEntity::StructField(
        StructField{
            name: get_field_name(name),
            type_name,
            comment: get_documentation(node),
            macros: yaserde_for_element(name, target_ns),
            subtypes: vec![]
        }
    )
}

fn parse_global_element(node: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let name = node
        .attr_name()
        .expect("Name required if the element is a child of the schema");

    if node.has_attribute(attribute::TYPE) {
        return RsEntity::Alias(
            Alias {
                name: match_type(name, target_ns).into(),
                original: match_type(node.attr_type().unwrap(), target_ns).into(),
                comment: get_documentation(node),
                subtypes: vec![],
            }
        );
    }
    let content_node = node.children().filter(|n| n.is_element()).last().expect("MUST HAVE CONTENT");
    let content = parse_node(&content_node, node, target_ns);

    RsEntity::Struct(
        Struct {
            name: match_type(name, target_ns).into(),
            fields: RefCell::new(vec![]),
            comment: get_documentation(node),
            subtypes: vec![content],
            macros: struct_macro(target_ns),
        }
    )
}

pub fn element_type(node: &Node, type_name: Cow<str>) -> String {
    let min = min_occurs(node);
    let max = max_occurs(node);
    match min {
        0 => match max {
            MaxOccurs::None => format!("Option<{}>", type_name),
            MaxOccurs::Unbounded => format!("Vec<{}>", type_name),
            MaxOccurs::Bounded(val) => {
                if val > 1 {
                    format!("Vec<{}>", type_name)
                } else {
                    type_name.to_string()
                }
            }
        },
        1=> match max {
            MaxOccurs::None => type_name.to_string(),
            MaxOccurs::Unbounded => format!("Vec<{}>", type_name),
            MaxOccurs::Bounded(val) => {
                if val > 1 {
                    format!("Vec<{}>", type_name)
                } else {
                    type_name.to_string()
                }
            }
        },
        _ => format!("Vec<{}>", type_name)
    }
}


