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

#[cfg(test)]
mod test {
    use crate::xml_to_xsd::ElementChildren;
    use crate::xsd_model::groups::facets::Facets;
    use crate::xsd_model::groups::simple_restriction_model::SimpleRestrictionModel;

    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
            r#"<root id="ID" a='b' b='a'>
                <simpleType id="STN">
                    <list itemType="ListOfType" />
                </simpleType>
                <minInclusive value="1"/>
                <maxInclusive value="5"/>
                <pattern value="[0-9]"/>
                </root>"#,
        )
        .unwrap();
        let root = doc.root_element();
        let mut iter = root.element_children();
        let res = SimpleRestrictionModel::parse(&mut iter).unwrap();

        assert_eq!(res.simple_type.unwrap().id.unwrap().0, "STN");
        assert_eq!(res.facets.len(), 3);
        if let Facets::MinInclusive(val) = &res.facets[0] {
            assert_eq!(val.value, "1")
        } else {
            panic!()
        }

        if let Facets::MaxInclusive(val) = &res.facets[1] {
            assert_eq!(val.value, "5")
        } else {
            panic!()
        }

        if let Facets::Pattern(val) = &res.facets[2] {
            assert_eq!(val.value, "[0-9]")
        } else {
            panic!()
        }
    }
}
