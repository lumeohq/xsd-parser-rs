use crate::xsd_model::complex_types::named_group::NamedGroup;
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::groups::schema_top::SchemaTop;
use crate::xsd_model::{
    TopLevelAttribute, TopLevelComplexType, TopLevelElement, TopLevelSimpleType,
};
use roxmltree::Node;
use std::rc::Rc;

impl<'a> SchemaTop<'a> {
    pub fn parse(node: Node<'a, '_>, element_type: ElementType) -> Result<Self, String> {
        let res = match element_type {
            ElementType::SimpleType => {
                SchemaTop::SimpleType(Rc::new(TopLevelSimpleType::parse(node)?))
            }
            ElementType::ComplexType => {
                SchemaTop::ComplexType(Rc::new(TopLevelComplexType::parse(node)?))
            }
            ElementType::Group => SchemaTop::Group(Rc::new(NamedGroup::parse(node)?)),
            //ElementType::AttributeGroup => SchemaTop::AttributeGroup(NamedAttributeGroup::parse(node)?),
            ElementType::Element => SchemaTop::Element(Rc::new(TopLevelElement::parse(node)?)),
            ElementType::Attribute => {
                SchemaTop::Attribute(Rc::new(TopLevelAttribute::parse(node)?))
            }
            ElementType::Notation => unimplemented!("Not presented in ONVIF"), //SchemaTop::Notation(Notation::parse(node)?),
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
