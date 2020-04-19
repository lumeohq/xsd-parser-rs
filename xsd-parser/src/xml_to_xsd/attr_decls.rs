use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::groups::attr_decls::AttrDecls;
use crate::xsd_model::{AnyAttribute, LocalAttribute};
use roxmltree::Node;

impl<'a> AttrDecls<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Attribute => res.attributes.push(LocalAttribute::parse(ch)?),
                ElementType::AttributeGroup => unimplemented!("AttributeGroupRef::parse"), //res.attribute_groups.push(AttributeGroupRef::parse(ch)?), //TODO:  AttributeGroupRef::parse
                ElementType::AnyAttribute => res.any_attribute = Some(AnyAttribute::parse(ch)?),
                _ => return Err(format!("Invalid node for xsd:attrDecls group: {:?}", ch)),
            }
        }
        Ok(res)
    }
}
