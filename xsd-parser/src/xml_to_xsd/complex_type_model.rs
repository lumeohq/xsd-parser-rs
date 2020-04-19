use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::groups::attr_decls::AttrDecls;
use crate::xsd_model::groups::complex_type_model::ComplexTypeModel;
use crate::xsd_model::groups::type_def_particle::TypeDefParticle;
use roxmltree::Node;

impl<'a> ComplexTypeModel<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let first_child = node
            .first_element_child()
            .ok_or_else(|| format!("complexTypeModel required: {:?}", node))?;
        Ok(match first_child.xsd_type()? {
            // ElementType::SimpleContent => {
            //     ComplexTypeModel::SimpleContent(SimpleContent::parse(first_child)?)
            // }
            // ElementType::ComplexContent => {
            //     ComplexTypeModel::ComplexContent(ComplexContent::parse(first_child)?)
            // }
            _ => ComplexTypeModel::Content(
                parse_type_def_particle(first_child),
                parse_attr_decls(node),
            ),
        })
    }
}

fn parse_type_def_particle(node: Node<'a, '_>) -> Option<TypeDefParticle<'a>> {
    TypeDefParticle::parse(node).ok()
}

fn parse_attr_decls<'a>(node: Node<'a, '_>) -> Vec<AttrDecls<'a>> {
    node.children()
        .filter(|n| n.is_element())
        .filter_map(|n| AttrDecls::parse(n).ok())
        .collect()
}
