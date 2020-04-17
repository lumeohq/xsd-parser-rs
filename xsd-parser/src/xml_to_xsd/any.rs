use crate::xml_to_xsd::utils::annotation_only;
use crate::xsd_model::Any;
use roxmltree::Node;

impl<'a> Any<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        res.annotation = annotation_only(node, "any")?;

        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.into()),
                "namespace" => res.namespace = attr.value(),
                "processContents" => res.process_contents = attr.value(),
                "minOccurs" => res.min_occurs = attr.value().parse()?,
                "maxOccurs" => res.max_occurs = attr.value().parse()?,
                _ => res.attributes.push(attr.clone()),
            };
        }
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::xsd_model::elements::any::Any;
    use crate::xsd_model::MaxOccurs;

    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
            r###"<any a='a' b='b' namespace="##any" processContents="lax" minOccurs="0" maxOccurs="unbounded" c='c'/>"###,
        )
        .unwrap();
        let root = doc.root_element();
        let res = Any::parse(root).unwrap();
        assert!(res.annotation.is_none());
        assert_eq!(res.attributes.len(), 3);
        assert_eq!(res.namespace, "##any");
        assert_eq!(res.process_contents, "lax");
        assert_eq!(res.min_occurs.0, 0);
        assert_eq!(res.max_occurs, MaxOccurs::Unbounded);
    }
}
