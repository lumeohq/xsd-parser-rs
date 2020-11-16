use std::cell::RefCell;

use roxmltree::Node;

use crate::parser::node_parser::parse_node;
use crate::parser::types::{RsEntity, Struct, StructField, StructFieldSource};
use crate::parser::utils::{
    attribute_groups_to_aliases, attributes_to_fields, get_documentation, get_parent_name,
};
use crate::parser::xsd_elements::{ElementType, XsdNode};

// A complex type can contain one and only one of the following elements,
// which determines the type of content allowed in the complex type.
const AVAILABLE_CONTENT_TYPES: [ElementType; 6] = [
    ElementType::All, //No in ONVIF
    ElementType::Choice,
    ElementType::ComplexContent,
    ElementType::Group, //No in ONVIF
    ElementType::Sequence,
    ElementType::SimpleContent,
];

pub fn parse_complex_type(node: &Node, parent: &Node) -> RsEntity {
    let name = if parent.xsd_type() == ElementType::Schema {
        node.attr_name()
            .expect("Name required if the complexType element is a child of the schema element")
    } else {
        get_parent_name(node)
    };

    let mut fields = attributes_to_fields(node);

    let content = node
        .children()
        .filter(|n| n.is_element() && AVAILABLE_CONTENT_TYPES.contains(&n.xsd_type()))
        .last();

    if content.is_none()
        || content
            .unwrap()
            .children()
            .filter(|n| n.is_element())
            .count()
            == 0
    {
        //No content (or empty), only attributes

        return RsEntity::Struct(Struct {
            fields: RefCell::new(fields),
            attribute_groups: RefCell::new(attribute_groups_to_aliases(node)),
            comment: get_documentation(node),
            subtypes: vec![],
            name: name.to_string(),
        });
    }
    let content_node = content.unwrap();

    let mut res = parse_node(&content_node, node);
    match &mut res {
        RsEntity::Struct(st) => {
            st.fields.borrow_mut().append(&mut fields);
            st.name = name.to_string();
        }
        RsEntity::Enum(en) => {
            en.name = format!("{}Choice", name);
            fields.push(StructField {
                name: en.name.clone(),
                type_name: en.name.clone(),
                source: StructFieldSource::Choice,
                ..Default::default()
            });
            en.subtypes = vec![RsEntity::Struct(Struct {
                name: name.to_string(),
                subtypes: vec![],
                comment: get_documentation(node),
                fields: RefCell::new(fields),
                attribute_groups: RefCell::new(attribute_groups_to_aliases(node)),
            })];
        }
        _ => (),
    };
    res
}
