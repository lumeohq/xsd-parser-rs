use roxmltree::Node;

use crate::parser::node_parser::parse_node;
use crate::parser::types::{Alias, RsEntity, Struct, StructField};
use crate::parser::utils::get_documentation;
use crate::parser::xsd_elements::{ElementType, XsdNode};

pub fn parse_attribute_group(node: &Node, parent: &Node) -> RsEntity {
    if parent.xsd_type() == ElementType::Schema {
        return parse_global_attribute_group(node);
    }

    let reference = node
        .attr_ref()
        .expect("Non-global attributeGroups must be references.")
        .to_string();

    RsEntity::Alias(Alias {
        name: reference.to_string(),
        original: reference,
        comment: get_documentation(node),
        ..Default::default()
    })
}

fn parse_global_attribute_group(node: &Node) -> RsEntity {
    let name = node
        .attr_name()
        .unwrap_or_else(|| panic!("Name attribute required. {:?}", node));

    let fields = attributes_to_fields(node);

    RsEntity::Struct(Struct {
        name: name.to_string(),
        fields: std::cell::RefCell::new(fields),
        ..Default::default()
    })
}

pub fn attributes_to_fields(node: &Node) -> Vec<StructField> {
    node.children()
        .filter(|n| {
            n.xsd_type() == ElementType::Attribute || n.xsd_type() == ElementType::AnyAttribute
        })
        .map(|n| match parse_node(&n, node) {
            RsEntity::StructField(sf) => sf,
            _ => unreachable!("Invalid attribute parsing: {:?}", n),
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::parser::attribute_group::parse_global_attribute_group;
    use crate::parser::types::RsEntity;
    use crate::parser::utils::find_child;

    #[test]
    fn test_global_attribute_with_nested_type() {
        let doc = roxmltree::Document::parse(
            r#"
        <xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"
           xmlns:xmime="http://www.w3.org/2005/05/xmlmime"
           targetNamespace="http://www.w3.org/2005/05/xmlmime" >

            <xs:attributeGroup name="contentGroup">
                <xs:attribute name="restricted">
                    <xs:simpleType>
                        <xs:restriction base="xs:string" >
                            <xs:minLength value="3" />
                        </xs:restriction>
                    </xs:simpleType>
                </xs:attribute>
                <xs:attribute name="simple" type="xs:string" />
            </xs:attributeGroup>
        </xs:schema>
        "#,
        )
        .unwrap();

        let schema = doc.root_element();
        let attribute = find_child(&schema, "attributeGroup").unwrap();
        match parse_global_attribute_group(&attribute) {
            RsEntity::Struct(ts) => {
                assert_eq!(ts.name, "contentGroup");
                assert_eq!(ts.fields.borrow().len(), 2);
            }
            _ => unreachable!("Test Failed!"),
        }
    }
}
