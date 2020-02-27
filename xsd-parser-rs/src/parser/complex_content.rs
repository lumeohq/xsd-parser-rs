use std::cell::RefCell;

use roxmltree::Node;

use crate::parser::constants::{attribute, tag};
use crate::parser::parser::parse_node;
use crate::parser::types::{RsEntity, Struct, StructField, StructFieldSource};
use crate::parser::utils::{
    any_attribute_field, attributes_to_fields, find_child, get_documentation,
};
use crate::parser::xsd_elements::{ElementType, ExtensionType, RestrictionType, XsdNode};

const AVAILABLE_CONTENT_TYPES: [ElementType; 6] = [
    ElementType::All, //No in ONVIF
    ElementType::Attribute,
    ElementType::AttributeGroup,
    ElementType::Choice,
    ElementType::Group, //No in ONVIF
    ElementType::Sequence,
];

pub fn parse_complex_content(node: &Node) -> RsEntity {
    let content = node
        .children()
        .filter(|n| n.is_element() && n.xsd_type() != ElementType::Annotation)
        .last()
        .expect("Content in complexContent required");

    match content.xsd_type() {
        ElementType::Restriction(r) => match r {
            RestrictionType::ComplexContent => complex_content_restriction(&content),
            _ => unreachable!("Invalid restriction type of complexContent {:?}", r),
        },
        ElementType::Extension(e) => match e {
            ExtensionType::ComplexContent => complex_content_extension(&content),
            _ => unreachable!("Invalid extension type of complexContent {:?}", e),
        },
        _ => unreachable!(
            "Complex content must be defined in one of the following ways: [Restriction, Extension]"
        ),
    }
}

fn complex_content_extension(node: &Node) -> RsEntity {
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

    let content = node
        .children()
        .filter(|n| {
            n.is_element()
                && n.xsd_type() != ElementType::Attribute
                && AVAILABLE_CONTENT_TYPES.contains(&n.xsd_type())
        })
        .last();

    if let Some(cont) = content {
        let mut res = parse_node(&cont, node);
        if let RsEntity::Struct(s) = &mut res {
            s.fields.borrow_mut().append(&mut fields);
            s.comment = get_documentation(node);
            return res;
        }
    }

    RsEntity::Struct(Struct {
        name: String::default(),
        subtypes: vec![],
        comment: get_documentation(node),
        fields: RefCell::new(fields),
    })
}

fn complex_content_restriction(node: &Node) -> RsEntity {
    unimplemented!("\n{:?}\n", node)
}
