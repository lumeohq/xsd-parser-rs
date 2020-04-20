use crate::xsd_model::groups::element_model::{ElementModel, ElementModelChoice};
use roxmltree::Node;
use crate::xml_to_xsd::{ElementChildren_, XsdNode};
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::LocalSimpleType;
use crate::xsd_model::complex_types::local_complex_type::LocalComplexType;


impl<'a> ElementModel<'a> {
    pub fn parse(node: Node<'a, 'a>) -> Result<Self, String> {
        let mut res = Self::default();
        for ch in node.element_children() {
            match ch.xsd_type()? {
                ElementType::SimpleType => res.choice = ElementModelChoice::SimpleType(LocalSimpleType::parse(ch)?),
                //ElementType::ComplexType=> res.choice = ElementModelChoice::ComplexType(LocalComplexType::parse(ch)?),
                ElementType::Unique |
                ElementType::Key |
                ElementType::KeyRef => {} //Not present in ONVIF //res.identity_constraints.push(IdentityConstraint::parse(ch)?),
                _ => return Err(format!("Invalid child node for xsd:elementModel group: {:?}", node))
            }
        }

        Ok(res)
    }
}