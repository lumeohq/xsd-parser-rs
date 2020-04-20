use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::groups::facets::Facets;
use crate::xsd_model::groups::simple_restriction_model::SimpleRestrictionModel;
use crate::xsd_model::LocalSimpleType;
use roxmltree::Node;

impl<'a> SimpleRestrictionModel<'a> {
    pub fn parse(iter: &mut impl Iterator<Item = Node<'a, 'a>>) -> Result<Self, String> {
        let mut res = Self::default();
        for ch in iter {
            match ch.xsd_type()? {
                ElementType::Annotation => {}
                ElementType::SimpleType => res.simple_type = Some(LocalSimpleType::parse(ch)?),
                x => {
                    let facet = Facets::parse(ch, x)?;
                    if let Some(x) = facet {
                        res.facets.push(x);
                    } else {
                        break;
                    }
                }
            };
        }

        Ok(res)
    }
}
