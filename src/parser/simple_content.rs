use std::cell::RefCell;

use roxmltree::{Namespace, Node};

use crate::parser::constants::{attribute, tag};
use crate::parser::types::{RsEntity, Struct, StructField};
use crate::parser::utils::{
    any_attribute_field, attributes_to_fields, find_child, get_documentation, match_type,
    struct_field_macros, struct_macro,
};
use crate::parser::xsd_elements::{ElementType, ExtensionType, RestrictionType, XsdNode};

pub fn parse_simple_content(node: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let content = node
        .children()
        .filter(|n| n.is_element() && n.xsd_type() != ElementType::Annotation)
        .last()
        .expect("Content in simpleContent required");

    match content.xsd_type() {
        ElementType::Restriction(r) => match r {
            RestrictionType::SimpleContent => simple_content_restriction(&content, target_ns),
            _ => unreachable!("Invalid restriction type of SimpleContent {:?}", r),
        },
        ElementType::Extension(e) => match e {
            ExtensionType::SimpleContent => simple_content_extension(&content, target_ns),
            _ => unreachable!("Invalid extension type of SimpleContent {:?}", e),
        },
        _ => unreachable!(
            "Simple content must be defined in one of the following ways: [Restriction, Extension]"
        ),
    }
}

fn simple_content_extension(node: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let base = match_type(
        node.attribute(attribute::BASE)
            .expect("The base value is required"),
        target_ns,
    );

    let mut fields = attributes_to_fields(node, target_ns);

    fields.push(StructField {
        name: tag::BASE.to_string(),
        type_name: base.to_string(),
        comment: get_documentation(node),
        macros: struct_field_macros("base"),
        subtypes: vec![],
    });

    if find_child(node, "anyAttribute").is_some() {
        fields.push(any_attribute_field())
    }

    RsEntity::Struct(Struct {
        name: String::default(),
        subtypes: vec![],
        comment: get_documentation(node),
        macros: struct_macro(target_ns),
        fields: RefCell::new(fields),
    })
}

fn simple_content_restriction(node: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    unimplemented!("\n{:?}  {:?}\n", node, target_ns)
}
