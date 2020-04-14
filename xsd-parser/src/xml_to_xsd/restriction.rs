use roxmltree::Node;
use crate::xsd_model::Restriction;
use crate::xsd_model::elements::ElementType;
use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::Annotation;
use crate::xsd_model::LocalSimpleType;
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::groups::simple_restriction_model::SimpleRestrictionModel;

impl<'a> Restriction<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();

        for ch in node.children().filter(|n| n.is_element()){
            match ch.xsd_type()? {
                ElementType::Annotation => res.annotation = Some(Annotation::parse(ch)?),
                _ => res.model = SimpleRestrictionModel::parse(ch)?
            };
        }
        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.into()),
                "base" => res.base = Some(QName::new(attr.value())),
                _ => res.attributes.push(attr.clone())
            };
        }
        Ok(res)
    }
}

impl<'a> SimpleRestrictionModel<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        use ElementType::*;
        let mut res = Self::default();

        for ch in node.children().filter(|n| n.is_element()){
            match ch.xsd_type()? {
                SimpleType => res.simple_type = Some(LocalSimpleType::parse(ch)?),
                //Facets
                MinExclusive => {}
                MinInclusive => {}
                MaxExclusive => {}
                MaxInclusive => {}
                TotalDigits => {}
                FractionDigits => {}
                Length => {}
                MinLength => {}
                MaxLength => {}
                Enumeration => {}
                WhiteSpace => {}
                Pattern => {}
                _ => return Err(format!("Invalid child node for xsd:restriction content: {:?}", node))
            };
        }

        Ok(res)
    }
}


#[cfg(test)]
mod test {
use crate::xsd_model::Restriction;
    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
  r#"<restriction id="ID" base="xsd:Type1" a='b' b='a'>
                    <minInclusive value="2"/>
                    <maxInclusive value="6"/>
            </restriction>"#
        ).unwrap();
        let root = doc.root_element();
        let res = Restriction::parse(root).unwrap();
        assert!(res.annotation.is_none());
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.id.unwrap().0, "ID");
        assert_eq!(res.base.as_ref().unwrap().name, "Type1");
        assert_eq!(res.base.as_ref().unwrap().prefix.unwrap(), "xsd");
        let model = &res.model;
        assert_eq!(model.facets.len(), 2);
    }
}