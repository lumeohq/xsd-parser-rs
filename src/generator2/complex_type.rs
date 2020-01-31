use roxmltree::Node;

use crate::generator2::types::{RsEntity, Struct};
use crate::generator2::utils::{any_attribute_field, find_child, get_documentation, get_parent_name, struct_macro, get_fields_from_attributes};
use crate::xsd::elements::{ElementType, XmlNode};
use crate::generator2::generator::parse_node;

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

pub fn parse_complex_type(node: &Node, parent: &Node, target_ns: Option<&roxmltree::Namespace>) -> RsEntity {
    let name = if parent.xsd_type() == ElementType::Schema {
        node.attribute("name")
            .expect("Name required if the complexType element is a child of the schema element")
    } else {
        get_parent_name(node)
    };

    let content = node
        .children()
        .filter(|n| n.is_element() && AVAILABLE_CONTENT_TYPES.contains(&n.xsd_type()))
        .last();

    let mut fields = get_fields_from_attributes(node, target_ns);
    match find_child(node, "anyAttribute") {
        Some(_) => fields.push(any_attribute_field()),
        None => (),
    };

    if content.is_none() ||
        content.unwrap().children().filter(|n| n.is_element()).count() == 0 {
        //No content (or empty), only attributes

        return RsEntity::Struct(Struct {
            fields,
            comment: get_documentation(node),
            macros: struct_macro(target_ns),
            subtypes: vec![],
            name: name.to_string(),
        });
    }
    let content_node = content.unwrap();

    let mut res = parse_node(&content_node, node, target_ns);
    match &mut res  {
        RsEntity::Struct( st) => {
            st.fields.append(&mut fields);
            st.name = name.to_string();
        },
        _ => () //TODO: enum from choice
    };
    res
}