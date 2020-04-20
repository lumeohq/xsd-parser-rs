use crate::xml_to_xsd::{ElementChildren_, XsdNode};
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::groups::attr_decls::AttrDecls;
use crate::xsd_model::groups::type_def_particle::TypeDefParticle;
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::{Annotation, ComplexRestriction};
use roxmltree::Node;

impl<'a> ComplexRestriction<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();

        let mut base = None;
        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.into()),
                "base" => base = Some(QName::new(attr.value())),
                _ => res.attributes.push(attr.clone()),
            };
        }
        res.base =
            base.ok_or_else(|| format!("Base required for xsd:extension element: {:?}", node))?;

        let mut iter = node.element_children().peekable();
        if iter.peek().is_none() {
            return Ok(res);
        }
        let mut elem_type = iter.peek().unwrap().xsd_type()?;

        if elem_type == ElementType::Annotation {
            res.annotation = Some(Annotation::parse(iter.next().unwrap())?);
        }

        if iter.peek().is_none() {
            return Ok(res);
        }

        elem_type = iter.peek().unwrap().xsd_type()?;

        if let Some(val) = TypeDefParticle::parse(*iter.peek().unwrap(), elem_type)? {
            res.type_def_particle = Some(val);
            if iter.next().is_none() {
                return Ok(res);
            }
        }

        res.attr_decls = AttrDecls::parse(iter)?;

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::xsd_model::groups::type_def_particle::TypeDefParticle;
    use crate::xsd_model::ComplexRestriction;

    #[test]
    fn test_empty() {
        let doc = roxmltree::Document::parse(
            r#"<xs:restriction base="tns:BarType" xmlns:xs="http://www.w3.org/2001/XMLSchema" a='a' b='b' id="ID">
                    </xs:restriction>"#,
        )
        .unwrap();
        let root = doc.root_element();
        let res = ComplexRestriction::parse(root).unwrap();
        assert!(res.annotation.is_none());
        assert_eq!(res.base.name, "BarType");
        assert_eq!(res.base.prefix, Some("tns"));
        assert!(res.type_def_particle.is_none());
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.id.unwrap().0, "ID");
        assert_eq!(res.attr_decls.attributes.len(), 0);
    }

    #[test]
    fn test_full() {
        let doc = roxmltree::Document::parse(
            r###"<xs:restriction base="tns:BarType" xmlns:xs="http://www.w3.org/2001/XMLSchema">
                        <xs:annotation>
							<xs:documentation>Text</xs:documentation>
						</xs:annotation>
                        <xs:sequence>
                            <xs:annotation>
                                <xs:documentation>Text</xs:documentation>
                            </xs:annotation>
                        </xs:sequence>
                        <xs:attribute name="Attr1" type="xs:unsignedInt" use="required"/>
                        <xs:attribute name="Attr2" type="xs:anyURI"/>
                        <xs:attribute name="Attr3" type="xs:unsignedInt" use="required"/>
                        <xs:anyAttribute namespace="##other" processContents="lax"/>
                    </xs:restriction>"###,
        )
        .unwrap();
        let root = doc.root_element();
        let res = ComplexRestriction::parse(root).unwrap();
        assert!(res.annotation.is_some());
        if let TypeDefParticle::Sequence(val) = res.type_def_particle.unwrap() {
            assert!(val.annotation.is_some());
        } else {
            panic!()
        }

        let attr = &res.attr_decls;
        assert_eq!(attr.attributes.len(), 3);
        assert_eq!(attr.any_attribute.as_ref().unwrap().process_contents, "lax");
    }

    #[test]
    fn test_only_attr_decls() {
        let doc = roxmltree::Document::parse(
            r###"<xs:restriction base="tns:BarType" xmlns:xs="http://www.w3.org/2001/XMLSchema">
                        <xs:attribute name="Attr1" type="xs:unsignedInt" use="required"/>
                        <xs:attribute name="Attr2" type="xs:anyURI"/>
                        <xs:attribute name="Attr3" type="xs:unsignedInt" use="required"/>
                        <xs:anyAttribute namespace="##other" processContents="lax"/>
                    </xs:restriction>"###,
        )
        .unwrap();
        let root = doc.root_element();
        let res = ComplexRestriction::parse(root).unwrap();
        assert!(res.annotation.is_none());
        assert!(res.type_def_particle.is_none());
        let attr = &res.attr_decls;
        assert_eq!(attr.attributes.len(), 3);
        assert_eq!(attr.any_attribute.as_ref().unwrap().process_contents, "lax");
    }
}
