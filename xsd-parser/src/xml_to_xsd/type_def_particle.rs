use crate::xsd_model::groups::type_def_particle::TypeDefParticle;
use crate::xsd_model::elements::ElementType;
use roxmltree::Node;
use crate::xsd_model::complex_types::named_group::NamedGroup;
use crate::xsd_model::complex_types::explicit_group::ExplicitGroup;

impl<'a> TypeDefParticle<'a> {
    pub fn parse(node: Node, element_type: ElementType) -> Result<Self, String> {
        Ok(match element_type {
            ElementType::Group => TypeDefParticle::Group(NamedGroup::parse(node)?),
            ElementType::All => unimplemented!("Not presented in ONVIF"), //TypeDefParticle::All(),
            ElementType::Choice => TypeDefParticle::Choice(ExplicitGroup::parse(node)?),
            ElementType::Sequence => TypeDefParticle::Sequence(ExplicitGroup::parse(node)?),
            _ => return Err(format!("Invalid node for xsd:typeDefParticle group: {:?}", node))
        })
    }
}