use crate::xml_to_xsd::{GroupErr, XsdNode};
use crate::xsd_model::groups::attr_decls::AttrDecls;
use crate::xsd_model::groups::complex_type_model::ComplexTypeModel;
use crate::xsd_model::groups::type_def_particle::TypeDefParticle;
use roxmltree::Node;
use crate::xml_to_xsd::utils::parse_attr_decls;
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::SimpleContent;

impl<'a> ComplexTypeModel<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let first_child = node
            .first_element_child()
            .ok_or_else(|| format!("complexTypeModel required: {:?}", node))?;
        Ok(match first_child.xsd_type()? {
            ElementType::SimpleContent => {
                ComplexTypeModel::SimpleContent(SimpleContent::parse(first_child)?)
            }
            // ElementType::ComplexContent => {
            //     ComplexTypeModel::ComplexContent(ComplexContent::parse(first_child)?)
            // }
            _ => ComplexTypeModel::Content(
                TypeDefParticle::parse(node).ok(),
                parse_attr_decls(node)?,
            ),
        })
    }
}
