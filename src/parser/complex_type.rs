use std::cell::RefCell;

use roxmltree::Node;

use crate::parser::parser::parse_node;
use crate::parser::types::{RsEntity, Struct, StructField};
use crate::parser::utils::{
    any_attribute_field, attributes_to_fields, find_child, get_documentation, get_field_name,
    get_parent_name, match_type, struct_macro,
};
use crate::parser::xsd_elements::{ElementType, XsdNode};

//A complex type can contain one and only one of the following elements,
// which determines the type of content allowed in the complex type.
const AVAILABLE_CONTENT_TYPES: [ElementType; 6] = [
    ElementType::All, //No in ONVIF
    ElementType::Choice,
    ElementType::ComplexContent,
    ElementType::Group, //No in ONVIF
    ElementType::Sequence,
    ElementType::SimpleContent,
];

pub fn parse_complex_type(
    node: &Node,
    parent: &Node,
    target_ns: Option<&roxmltree::Namespace>,
) -> RsEntity {
    let name = if parent.xsd_type() == ElementType::Schema {
        node.attr_name()
            .expect("Name required if the complexType element is a child of the schema element")
    } else {
        get_parent_name(node)
    };

    let content = node
        .children()
        .filter(|n| n.is_element() && AVAILABLE_CONTENT_TYPES.contains(&n.xsd_type()))
        .last();

    let mut fields = attributes_to_fields(node, target_ns);
    match find_child(node, "anyAttribute") {
        Some(_) => fields.push(any_attribute_field()),
        None => (),
    };

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
            comment: get_documentation(node),
            macros: struct_macro(target_ns),
            subtypes: vec![],
            name: name.to_string(),
        });
    }
    let content_node = content.unwrap();

    let mut res = parse_node(&content_node, node, target_ns);
    match &mut res {
        RsEntity::Struct(st) => {
            st.fields.borrow_mut().append(&mut fields);
            st.name = name.to_string();
        }
        RsEntity::Enum(en) => {
            en.name = format!("{}Choice", name);
            fields.push(StructField {
                name: get_field_name(en.name.as_str()),
                type_name: match_type(en.name.as_str(), target_ns).into(),
                comment: None,
                macros: "//TODO: add yaserde macros\n".to_string(),
                subtypes: vec![],
            });
            en.subtypes = vec![RsEntity::Struct(Struct {
                name: name.to_string(),
                subtypes: vec![],
                macros: struct_macro(target_ns),
                comment: get_documentation(node),
                fields: RefCell::new(fields),
            })];
        }
        _ => (),
    };
    res
}
