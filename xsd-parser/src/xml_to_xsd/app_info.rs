use crate::xsd_model::elements::app_info::AppInfo;
use roxmltree::Node;

impl<'a> AppInfo<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<AppInfo<'a>, String> {
        let mut res = AppInfo::default();
        res.text = node.text();
        res.elements = node.children().filter(|n| n.is_element()).collect();
        for attr in node.attributes() {
            match attr.name() {
                "source" => res.source = Some(attr.into()),
                _ => res.attributes.push(attr.clone()),
            };
        }

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::xsd_model::elements::app_info::AppInfo;

    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
            r#"<appInfo source="http://ya.com" xml:lang="us" a='a' b='a'>
            A string
            <el>Some element</el>
            </appInfo>"#,
        )
        .unwrap();
        let root = doc.root_element();
        let res = AppInfo::parse(root).unwrap();
        assert_eq!(res.text.unwrap().trim(), "A string");
        assert_eq!(res.source.unwrap().0, "http://ya.com");
        assert_eq!(res.attributes.len(), 3);
        assert_eq!(res.elements.len(), 1);
        assert_eq!(res.elements[0].text().unwrap(), "Some element");
    }
}
