use roxmltree::Node;

use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::elements::ElementType;
use std::str::ParseBoolError;

use crate::xsd_model::complex_types::explicit_group::ExplicitGroup;
use crate::xsd_model::groups::nested_particle::NestedParticle;
use crate::xsd_model::Annotation;

impl<'a> ExplicitGroup<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();

        for ch in node.children().filter(|n| n.is_element()){
            match ch.xsd_type()? {
                ElementType::Annotation => res.annotation = Some(Annotation::parse(ch)?),
                _ => res.nested_particle.push(NestedParticle::parse(ch)?),
            }
        }

        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.into()),
                "minOccurs" => res.min_occurs = attr.value().parse()?,
                "maxOccurs" => res.max_occurs = attr.value().parse()?,
                _ => res.attributes.push(attr.clone()),
            };
        }

        Ok(res)
    }
}