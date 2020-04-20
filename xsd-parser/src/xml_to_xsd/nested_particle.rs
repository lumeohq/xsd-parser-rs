use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::complex_types::explicit_group::ExplicitGroup;
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::groups::nested_particle::NestedParticle;
use crate::xsd_model::{Any, LocalElement};
use roxmltree::Node;

impl<'a> NestedParticle<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let res = match node.xsd_type()? {
            ElementType::Element => NestedParticle::Element(Box::new(LocalElement::parse(node)?)),
            // ElementType::Group => Self::Group(::parse(node)?),
            ElementType::Choice => NestedParticle::Choice(ExplicitGroup::parse(node)?),
            ElementType::Sequence => NestedParticle::Sequence(ExplicitGroup::parse(node)?),
            ElementType::Any => NestedParticle::Any(Any::parse(node)?),
            _ => {
                return Err(format!(
                    "Error NestedParticle parsing. Invalid node: {:?}",
                    node
                ))
            }
        };

        Ok(res)
    }
}
