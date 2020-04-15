use crate::xsd_model::Documentation;
use roxmltree::Node;

impl<'a> Documentation<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Documentation<'a>, String> {
        let mut res = Documentation::default();
        res.text = node.text();
        res.elements = node.children().filter(|n| n.is_element()).collect();
        for attr in node.attributes() {
            match attr.name() {
                "source" => res.source = Some(attr.into()),
                "lang" => res.lang = Some(attr.into()),
                _ => res.attributes.push(attr.clone()),
            };
        }

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::xsd_model::Documentation;

    #[test]
    fn test_documentation_parse() {
        let doc = roxmltree::Document::parse(
            r#"<documentation source="http://ya.com" xml:lang="us" a='a' b='a'>
            A string
            <el>Some element</el>
            </documentation>"#,
        )
        .unwrap();
        let root = doc.root_element();
        let res = Documentation::parse(root).unwrap();
        assert_eq!(res.text.unwrap().trim(), "A string");
        assert_eq!(res.source.unwrap().0, "http://ya.com");
        assert_eq!(res.lang.unwrap().0, "us");
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.elements.len(), 1);
        assert_eq!(res.elements[0].text().unwrap(), "Some element");
    }
}
