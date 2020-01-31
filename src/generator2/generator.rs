use roxmltree::Namespace;

use crate::generator2::complex_type::parse_complex_type;
use crate::generator2::element::parse_element;
use crate::generator2::simple_type::parse_simple_type;
use crate::generator2::types::{RsEntity, File, Import, StructField};
use crate::generator2::utils::{target_namespace, get_documentation};
use crate::xsd::elements::{ElementType, XmlNode};
use crate::generator2::sequence::parse_sequence;
use crate::generator2::simple_content::parse_simple_content;
use crate::generator2::complex_content::parse_complex_content;
use crate::generator2::choice::parse_choice;

pub fn parse(text: &str) {
    let doc = match roxmltree::Document::parse(&text) {
        Ok(doc) => doc,
        Err(e) => {
            panic!("Error: {}.", e);
        }
    };
    let root =  doc.root();

    for node in root.children().filter(|e| e.is_element()) {
        println!("{}", parse_node(&node, &root, None));
    }


    //TODO: add return value
}

pub fn parse_schema(schema: &roxmltree::Node<'_, '_>) -> RsEntity {
    RsEntity::File(
        File{
            name: "".into(),
            namespace: None,
            types: schema
                .children()
                .filter(|n| n.is_element())
                .map(|node| parse_node(&node, schema, target_namespace(&schema)))
                .collect()
        }
    )
}

pub fn parse_node(node: &roxmltree::Node<'_, '_>, parent: &roxmltree::Node, tn: Option<&Namespace>) -> RsEntity {
    use ElementType::*;

    match node.xsd_type() {
        Schema => parse_schema(node),
        SimpleType => parse_simple_type(node, parent, tn),
        ComplexType => parse_complex_type(node, parent, tn),
        Element => parse_element(node, parent, tn),
        Import | Include => parse_import(node),
        Any => parse_any(node),
        Sequence => parse_sequence(node, parent, tn),
        SimpleContent => parse_simple_content(node, tn),
        ComplexContent => parse_complex_content(node, tn),
        Choice => parse_choice(node, tn),

        _ => {unreachable!("{:?}", node);},
    }
}


// Stubs
fn parse_import(node: &roxmltree::Node) -> RsEntity {
    RsEntity::Import(
        Import{
            name: node.attribute("namespace").unwrap_or("").into(),
            location: node.attribute("schemaLocation").unwrap_or("").into()
        }
    )
}

fn parse_any(node: &roxmltree::Node) -> RsEntity {
    RsEntity::StructField(
        StructField{
            name: "any".to_string(),
            type_name: "AnyElement".to_string(),
            macros: "// ".to_string(),
            subtypes: vec![],
            comment: get_documentation(node)
        }
    )
}
