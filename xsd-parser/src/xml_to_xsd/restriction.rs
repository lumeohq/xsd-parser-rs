use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::groups::facets::Facets;
use crate::xsd_model::groups::simple_restriction_model::SimpleRestrictionModel;
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::Annotation;
use crate::xsd_model::LocalSimpleType;
use crate::xsd_model::Restriction;
use roxmltree::Node;

impl<'a> Restriction<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        for ch in node.children().filter(|n| n.is_element()) {
            if let ElementType::Annotation = ch.xsd_type()? {
                res.annotation = Some(Annotation::parse(ch)?)
            }
        }

        res.model = SimpleRestrictionModel::parse(node)?;

        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.into()),
                "base" => res.base = Some(QName::new(attr.value())),
                _ => res.attributes.push(attr.clone()),
            };
        }
        Ok(res)
    }
}

impl<'a> SimpleRestrictionModel<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        use ElementType::*;
        let mut res = Self::default();
        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                Annotation => {}
                SimpleType => res.simple_type = Some(LocalSimpleType::parse(ch)?),
                _ => res.facets.push(Facets::parse(ch).map_err(|_| {
                    format!("Invalid child node for xsd:restriction content: {:?}", node)
                })?),
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
            r#"<restriction xmlns:xsd="http://www.w3.org/2001/XMLSchema" id="ID" base="xsd:Type1" a='b' b='a'>
                    <xsd:minExclusive value="2"/>
                    <xsd:minInclusive value="1"/>
                    <xsd:maxExclusive value="6"/>
                    <xsd:maxInclusive value="5"/>

                    <xsd:totalDigits value="1"/>
                    <xsd:fractionDigits value="1"/>
                    <xsd:length value="1"/>
                    <xsd:minLength value="1"/>
                    <xsd:maxLength value="1"/>
                    <xsd:enumeration value="4"/>
                    <xsd:whiteSpace value="collapse"/>
                    <xsd:pattern value="[2-5]"/>
            </restriction>"#,
        )
        .unwrap();
        let root = doc.root_element();
        let res = Restriction::parse(root).unwrap();
        assert!(res.annotation.is_none());
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.id.unwrap().0, "ID");
        assert_eq!(res.base.as_ref().unwrap().name, "Type1");
        assert_eq!(res.base.as_ref().unwrap().prefix.unwrap(), "xsd");
        let model = &res.model;
        assert_eq!(model.facets.len(), 12);
    }
}
