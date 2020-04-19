use crate::xml_to_xsd::utils::annotation_only;
use crate::xsd_model::AnyAttribute;
use roxmltree::Node;

impl<'a> AnyAttribute<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        res.annotation = annotation_only(node, "any")?;

        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.into()),
                "namespace" => res.namespace = attr.value(),
                "processContents" => res.process_contents = attr.value(),
                _ => res.attributes.push(attr.clone()),
            };
        }
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::xsd_model::AnyAttribute;

    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
            r###"<anyAttribute a='a' b='b' namespace="##any" processContents="lax" c='c'/>"###,
        )
        .unwrap();
        let root = doc.root_element();
        let res = AnyAttribute::parse(root).unwrap();
        assert!(res.annotation.is_none());
        assert_eq!(res.attributes.len(), 3);
        assert_eq!(res.namespace, "##any");
        assert_eq!(res.process_contents, "lax");
    }
}
