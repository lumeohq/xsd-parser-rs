use crate::xsd_model::complex_types::named_group::NamedGroup;
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::groups::schema_top::SchemaTop;
use crate::xsd_model::{TopLevelAttribute, TopLevelComplexType, TopLevelSimpleType};
use roxmltree::Node;

impl<'a> SchemaTop<'a> {
    pub fn parse(node: Node<'a, '_>, element_type: ElementType) -> Result<Self, String> {
        let res = match element_type {
            ElementType::SimpleType => {
                SchemaTop::SimpleType(Box::new(TopLevelSimpleType::parse(node)?))
            }
            ElementType::ComplexType => {
                SchemaTop::ComplexType(Box::new(TopLevelComplexType::parse(node)?))
            }
            ElementType::Group => SchemaTop::Group(Box::new(NamedGroup::parse(node)?)),
            //ElementType::AttributeGroup => SchemaTop::AttributeGroup(NamedAttributeGroup::parse(node)?),
            //ElementType::Element => SchemaTop::Element(TopLevelElement::parse(node)?),
            ElementType::Attribute => {
                SchemaTop::Attribute(Box::new(TopLevelAttribute::parse(node)?))
            }
            //ElementType::Notation => SchemaTop::Notation(Notation::parse(node)?),
            _ => {
                return Err(format!(
                    "Error xsd:schemaTop parsing. Invalid node: {:?}",
                    node
                ))
            }
        };

        Ok(res)
    }
}
