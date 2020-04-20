use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::groups::attr_decls::AttrDecls;
use crate::xsd_model::{AnyAttribute, LocalAttribute};
use roxmltree::Node;

impl<'a> AttrDecls<'a> {
    pub fn parse(iter: impl Iterator<Item = Node<'a, 'a>>) -> Result<Self, String> {
        let mut res = Self::default();
        for ch in iter {
            match ch.xsd_type()? {
                ElementType::Attribute => res.attributes.push(LocalAttribute::parse(ch)?),
                ElementType::AttributeGroup => unimplemented!("AttributeGroupRef::parse"), //res.attribute_groups.push(AttributeGroupRef::parse(ch)?), //TODO:  AttributeGroupRef::parse
                ElementType::AnyAttribute => res.any_attribute = Some(AnyAttribute::parse(ch)?),
                _ => break,
            }
        }
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::xml_to_xsd::{ElementChildren, ElementChildren_};
    use crate::xsd_model::groups::attr_decls::AttrDecls;

    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
            r#"<root xmlns:xs="http://www.w3.org/2001/XMLSchema">
                   <xs:attribute name="Attr1" type="xs:boolean" />
                   <xs:attribute name="Attr2" type="xs:boolean" />
                   <xs:attribute name="Attr3" type="xs:boolean" />
                   <xs:anyAttribute processContents="lax"/>
                   <xs:any name="ThisIsTheEnd" />
                   </root>"#,
        )
        .unwrap();

        let root = doc.root_element();
        let mut iter = root.element_children();

        let res = AttrDecls::parse(&mut iter).unwrap();
        assert_eq!(res.attribute_groups.len(), 0);
        assert_eq!(res.attributes.len(), 3);
        assert!(res.any_attribute.is_some());
    }
}
