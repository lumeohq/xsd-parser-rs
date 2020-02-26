use std::cell::RefCell;

use roxmltree::Node;

use crate::parser::constants::{attribute, tag};
use crate::parser::types::{RsEntity, Struct, StructField, StructFieldSource};
use crate::parser::utils::{
    any_attribute_field, attributes_to_fields, find_child, get_documentation,
};
use crate::parser::xsd_elements::{ElementType, ExtensionType, RestrictionType, XsdNode};

pub fn parse_simple_content(node: &Node) -> RsEntity {
    let content = node
        .children()
        .filter(|n| n.is_element() && n.xsd_type() != ElementType::Annotation)
        .last()
        .expect("Content in simpleContent required");

    match content.xsd_type() {
        ElementType::Restriction(r) => match r {
            RestrictionType::SimpleContent => simple_content_restriction(&content),
            _ => unreachable!("Invalid restriction type of SimpleContent {:?}", r),
        },
        ElementType::Extension(e) => match e {
            ExtensionType::SimpleContent => simple_content_extension(&content),
            _ => unreachable!("Invalid extension type of SimpleContent {:?}", e),
        },
        _ => unreachable!(
            "Simple content must be defined in one of the following ways: [Restriction, Extension]"
        ),
    }
}

fn simple_content_extension(node: &Node) -> RsEntity {
    let base = node
        .attribute(attribute::BASE)
        .expect("The base value is required");

    let mut fields = attributes_to_fields(node);

    fields.push(StructField {
        name: tag::BASE.to_string(),
        type_name: base.to_string(),
        comment: get_documentation(node),
        source: StructFieldSource::Base,
        ..Default::default()
    });

    if find_child(node, "anyAttribute").is_some() {
        fields.push(any_attribute_field())
    }

    RsEntity::Struct(Struct {
        name: String::default(),
        subtypes: vec![],
        comment: get_documentation(node),
        fields: RefCell::new(fields),
    })
}

fn simple_content_restriction(node: &Node) -> RsEntity {
    unimplemented!("\n{:?}\n", node)
}
