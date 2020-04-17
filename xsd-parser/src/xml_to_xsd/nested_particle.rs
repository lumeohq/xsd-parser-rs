use crate::xsd_model::groups::nested_particle::NestedParticle;
use roxmltree::Node;
use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::elements::ElementType;

impl<'a> NestedParticle<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let res = match node.xsd_type()? {
            // ElementType::Element => Self::Element(Facet::parse(node)?),
            // ElementType::Group => Self::Group(Facet::parse(node)?),
            // ElementType::Choice => Self::Choice(Facet::parse(node)?),
            // ElementType::Sequence => Self::Sequence(Facet::parse(node)?),
            // ElementType::Any => Self::Any(TotalDigits::parse(node)?),
            _ => return Err(format!("Error NestedParticle parsing. Invalid node: {:?}", node)),
        };

        Ok(res)
    }
}