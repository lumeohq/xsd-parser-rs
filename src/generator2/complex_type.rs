use roxmltree::Node;

use crate::generator2::types::{RsType, Struct, StructField};
use crate::generator2::utils::{any_attribute_field, find_child, get_documentation, get_field_name, get_parent_name, match_type, struct_field_macros, struct_macro};
use crate::xsd::elements::{ElementType, ExtensionType, Name, RestrictionType, XmlNode};

//A complex type can contain one and only one of the following elements,
// which determines the type of content allowed in the complex type.
const AVAILABLE_CONTENT_TYPES: [ElementType; 6] = [
    ElementType::SimpleContent,
    ElementType::ComplexContent,
    ElementType::Group,
    ElementType::All,
    ElementType::Choice,
    ElementType::Sequence,
];

pub fn parse_complex_type(node: &Node, parent: &Node, target_ns: Option<&roxmltree::Namespace>) -> RsType {
    let name = if parent.xsd_type() == ElementType::Schema {
        node.attribute("name")
            .expect("Name required if the complexType element is a child of the schema element")
            .to_string()
    } else {
        get_parent_name(Some(*parent))
    };

    let content = node
        .children()
        .filter(|n| n.is_element() && AVAILABLE_CONTENT_TYPES.contains(&n.xsd_type()))
        .last();

    if content.is_none() ||
        content.unwrap().children().filter(|n| n.is_element()).count() == 0 {
        //No content (or empty), only attributes
        let mut fields = fields_from_attributes(node, target_ns);
        match find_child(node, "anyAttribute") {
            Some(_) => fields.push(any_attribute_field()),
            None => (),
        };
        return RsType::Struct(Struct {
            fields,
            comment: get_documentation(node),
            macros: struct_macro(target_ns),
            subtypes: vec![],
            name,
        });
    }
    let content_node = content.unwrap();

    match content_node.xsd_type() {
        ElementType::SimpleContent => parse_simple_content(&content_node, name.as_str(), target_ns),
        // ElementType::Sequence => parse_sequence(&content_node, name.as_str(), target_ns),
        //        ElementType::ComplexContent => unimplemented!(),
        //        ElementType::Group => unimplemented!(),
        //        ElementType::All => unimplemented!(),
        //        ElementType::Choice => unimplemented!(),
        //        _ => panic!("Invalid content type of complexType {:?}", content_node.xsd_type()),
        _ => {
            return RsType::Struct(Struct {
                fields: fields_from_attributes(node, target_ns),
                comment: get_documentation(node),
                macros: struct_macro(target_ns),
                subtypes: vec![],
                name,
            });
        }
    }
}

fn parse_simple_content(node: &Node, name: &str, target_ns: Option<&roxmltree::Namespace>) -> RsType {
    let content = node
        .children()
        .filter(|n| n.is_element() && n.xsd_type() != ElementType::Annotation)
        .last()
        .expect(
            "Simple content must be defined in one of the following ways: [Restriction, Extension]",
        );

    match content.xsd_type() {
        ElementType::Restriction(r) => match r {
            RestrictionType::SimpleContent => unimplemented!("No in ONVIF"),
            _ => unreachable!("Invalid restriction type of SimpleContent {:?}", r),
        },
        ElementType::Extension(e) => match e {
            ExtensionType::SimpleContent => simple_content_extension(&content, name, target_ns),
            _ => unreachable!("Invalid extension type of SimpleContent {:?}", e),
        },
        _ => unreachable!(),
    }
}

fn simple_content_extension(node: &Node, name: &str, target_ns: Option<&roxmltree::Namespace>) -> RsType {
    let base = match_type(
        node.attribute("base").expect("The base value is required"),
        target_ns,
    );

    let struct_name = match_type(name, target_ns);

    let mut fields = fields_from_attributes(node, target_ns);

    fields.push(
        StructField {
            name: "base".to_string(),
            type_name: base.to_string(),
            comment: get_documentation(node),
            macros: struct_field_macros("base"),
        }
    );

    match find_child(node, "anyAttribute") {
        Some(_) => fields.push(any_attribute_field()),
        None => (),
    };

    RsType::Struct(Struct {
        name: struct_name.to_string(),
        subtypes: vec![],
        comment: get_documentation(node),
        macros: struct_macro(target_ns),
        fields,
    })
}

fn fields_from_attributes(node: &Node, target_ns: Option<&roxmltree::Namespace>) -> Vec<StructField> {
    node.children()
        .filter(|n| n.is_element() && n.xsd_type() == ElementType::Attribute)
        .map(|n| attribute_to_field(&n, target_ns))
        .collect()
}

fn attribute_to_field(node: &Node, target_ns: Option<&roxmltree::Namespace>) -> StructField {
    let name = get_field_name(
        node.attribute("name").or(node.attribute("ref"))
            .expect("All attributes have name or ref in Onvif"),
    );

    StructField {
        macros: struct_field_macros(name.as_str()),
        type_name: match_type(
            node.attribute("type")
                .or(node.attribute("ref"))
                .unwrap_or("()"),
            target_ns,
        )
        .to_string(),
        comment: get_documentation(node),
        name,
    }
}
