use std::cell::RefCell;

use roxmltree::{Namespace, Node};

use crate::parser::constants::{attribute, tag};
use crate::parser::parser::parse_node;
use crate::parser::types::{RsEntity, Struct, StructField, StructFieldSource};
use crate::parser::utils::{
    any_attribute_field, attributes_to_fields, find_child, get_documentation, match_type,
    struct_macro,
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

pub fn parse_complex_content(node: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    let content = node
        .children()
        .filter(|n| n.is_element() && n.xsd_type() != ElementType::Annotation)
        .last()
        .expect("Content in complexContent required");

    match content.xsd_type() {
        ElementType::Restriction(r) => match r {
            RestrictionType::ComplexContent => complex_content_restriction(&content, target_ns),
            _ => unreachable!("Invalid restriction type of complexContent {:?}", r),
        },
        ElementType::Extension(e) => match e {
            ExtensionType::ComplexContent => complex_content_extension(&content, target_ns),
            _ => unreachable!("Invalid extension type of complexContent {:?}", e),
        },
        _ => unreachable!(
            "Complex content must be defined in one of the following ways: [Restriction, Extension]"
        ),
    }
}

fn complex_content_extension(node: &Node, target_ns: Option<&Namespace>) -> RsEntity {
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
        macros: String::new(),
        subtypes: vec![],
        source: StructFieldSource::Base
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
        let mut res = parse_node(&cont, node, target_ns);
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
        macros: struct_macro(target_ns),
        fields: RefCell::new(fields),
    })
}

fn complex_content_restriction(node: &Node, target_ns: Option<&Namespace>) -> RsEntity {
    unimplemented!("\n{:?}  {:?}\n", node, target_ns)
}
