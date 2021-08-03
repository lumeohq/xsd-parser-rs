use roxmltree::Node;

use crate::parser::node_parser::parse_node;
use crate::parser::types::RsFile;
use crate::parser::utils::target_namespace;
use crate::parser::xsd_elements::{ElementType, XsdNode};

pub fn parse_schema<'input>(schema: &Node<'_, 'input>) -> RsFile<'input> {
    RsFile {
        name: "".into(),
        namespace: None,
        target_ns: target_namespace(schema).cloned(),
        xsd_ns: schema
            .namespaces()
            .iter()
            .rev()
            .find(|a| a.uri() == "http://www.w3.org/2001/XMLSchema")
            .cloned(),
        types: schema
            .children()
            .filter(|n| {
                n.is_element()
                    && n.xsd_type() != ElementType::Annotation
                    && n.xsd_type() != ElementType::AttributeGroup
            })
            .map(|node| parse_node(&node, schema))
            .collect(),
        attribute_groups: schema
            .children()
            .filter(|n| n.is_element() && n.xsd_type() == ElementType::AttributeGroup)
            .map(|node| parse_node(&node, schema))
            .collect(),
    }
}

#[cfg(test)]
mod test {
    use crate::parser::schema::parse_schema;

    #[test]
    fn test_multiple_xsd_ns() {
        let doc = roxmltree::Document::parse(
            r#"
    <xs:schema
        xmlns:tt="http://www.onvif.org/ver10/schema"
        xmlns="http://www.w3.org/2001/XMLSchema"
        xmlns:xs="http://www.w3.org/2001/XMLSchema"
        xmlns:xsd="http://www.w3.org/2001/XMLSchema"
        targetNamespace="http://www.onvif.org/ver10/schema"
        >
    </xs:schema>
                "#,
        )
        .unwrap();

        let res = parse_schema(&doc.root_element());
        assert_eq!(res.xsd_ns.unwrap().name().unwrap(), "xsd");
    }
}
