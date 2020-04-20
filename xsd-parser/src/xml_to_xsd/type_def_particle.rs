use crate::xsd_model::complex_types::explicit_group::ExplicitGroup;
use crate::xsd_model::complex_types::named_group::NamedGroup;
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::groups::type_def_particle::TypeDefParticle;
use roxmltree::Node;

impl<'a> TypeDefParticle<'a> {
    pub fn parse(node: Node<'a, '_>, element_type: ElementType) -> Result<Option<Self>, String> {
        Ok(Some(match element_type {
            ElementType::Group => TypeDefParticle::Group(NamedGroup::parse(node)?),
            ElementType::All => unimplemented!("Not presented in ONVIF"), //TypeDefParticle::All(),
            ElementType::Choice => TypeDefParticle::Choice(ExplicitGroup::parse(node)?),
            ElementType::Sequence => TypeDefParticle::Sequence(ExplicitGroup::parse(node)?),
            _ => return Ok(None),
        }))
    }
}
