use crate::xml_to_xsd::utils::annotation_first;
use crate::xml_to_xsd::ElementChildren;
use crate::xsd_model::groups::simple_restriction_model::SimpleRestrictionModel;
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::Restriction;
use roxmltree::Node;

impl<'a> Restriction<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        res.annotation = annotation_first(node)?;
        let skip = if res.annotation.is_some() { 1 } else { 0 };
        let mut iter = node.element_children().skip(skip);
        res.model = SimpleRestrictionModel::parse(&mut iter)?;
        if let Some(n) = iter.next() {
            return Err(format!(
                "Invalid child node for xsd:restriction element: {:?}",
                n
            ));
        }

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

#[cfg(test)]
mod test {
    use crate::xsd_model::Restriction;
    #[test]
    fn test_parse_restriction() {
        let doc = roxmltree::Document::parse(
            r#"<restriction xmlns:xsd="http://www.w3.org/2001/XMLSchema" id="ID" base="xsd:Type1" a='b' b='a'>
                    <xsd:annotation>
						<xsd:documentation>Text</xsd:documentation>
					</xsd:annotation>

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
        assert!(res.annotation.is_some());
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.id.unwrap().0, "ID");
        assert_eq!(res.base.as_ref().unwrap().name, "Type1");
        assert_eq!(res.base.as_ref().unwrap().prefix.unwrap(), "xsd");
        let model = &res.model;
        assert_eq!(model.facets.len(), 12);
    }

    #[test]
    fn test_parse_restriction_invalid_node() {
        let doc = roxmltree::Document::parse(
            r#"<restriction xmlns:xsd="http://www.w3.org/2001/XMLSchema" id="ID" base="xsd:Type1" a='b' b='a'>
                    <xsd:minExclusive value="2"/>
                    <xsd:minInclusive value="1"/>
                    <xsd:maxExclusive value="6"/>
                    <xsd:maxInclusive value="5"/>
                    <xsd:annotation>
						<xsd:documentation>Text</xsd:documentation>
					</xsd:annotation>
					<xsd:annotation>
						<xsd:documentation>Text</xsd:documentation>
					</xsd:annotation>
            </restriction>"#,
        )
        .unwrap();
        let root = doc.root_element();
        assert!(Restriction::parse(root).is_err());
    }
}
