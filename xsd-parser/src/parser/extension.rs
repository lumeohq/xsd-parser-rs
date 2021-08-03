use crate::parser::constants::{attribute, tag};
use crate::parser::node_parser::parse_node;
use crate::parser::types::{RsEntity, Struct, StructField, StructFieldSource};
use crate::parser::utils::{
    attribute_groups_to_aliases, attributes_to_fields, get_base, get_documentation,
};
use crate::parser::xsd_elements::{ElementType, ExtensionType, XsdNode};
use roxmltree::Node;
use std::cell::RefCell;

const AVAILABLE_CONTENT_TYPES: [ElementType; 6] = [
    ElementType::All, // Not presented in ONVIF
    ElementType::Attribute,
    ElementType::AttributeGroup,
    ElementType::Choice,
    ElementType::Group, // Not presented in ONVIF
    ElementType::Sequence,
];

pub fn parse_extension(node: &Node, _: &Node) -> RsEntity {
    use ElementType::Extension;
    match node.xsd_type() {
        Extension(ExtensionType::SimpleContent) => simple_content_extension(node),
        Extension(ExtensionType::ComplexContent) => complex_content_extension(node),
        _ => unreachable!("Invalid extension node: {:#?}", node),
    }
}

fn simple_content_extension(node: &Node) -> RsEntity {
    let base = get_base(node);
    let mut fields = attributes_to_fields(node);

    fields.push(StructField {
        name: tag::BASE.to_string(),
        type_name: base.to_string(),
        comment: get_documentation(node),
        source: StructFieldSource::Base,
        ..Default::default()
    });

    RsEntity::Struct(Struct {
        name: String::default(),
        subtypes: vec![],
        comment: get_documentation(node),
        fields: RefCell::new(fields),
        attribute_groups: RefCell::new(attribute_groups_to_aliases(node)),
    })
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
        comment: get_documentation(node),
        fields: RefCell::new(fields),
        attribute_groups: RefCell::new(attribute_groups_to_aliases(node)),
        ..Default::default()
    })
}
