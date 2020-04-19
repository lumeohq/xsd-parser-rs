use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::groups::facets::Facets;
use crate::xsd_model::groups::simple_restriction_model::SimpleRestrictionModel;
use crate::xsd_model::LocalSimpleType;
use roxmltree::Node;

impl<'a> SimpleRestrictionModel<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Annotation => {}
                ElementType::SimpleType => res.simple_type = Some(LocalSimpleType::parse(ch)?),
                x => res.facets.push(Facets::parse(ch, x).map_err(|_| {
                    format!("Invalid child node for xsd:restriction content: {:?}", ch)
                })?),
            };
        }

        Ok(res)
    }
}
