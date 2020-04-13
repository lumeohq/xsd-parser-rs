use crate::xsd_model::elements::annotation::Annotation;
use roxmltree::Node;
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::elements::app_info::AppInfo;
use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::elements::documentation::Documentation;

impl<'a> Annotation<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Annotation<'a>, String> {
        let mut res = Annotation::default();

        for ch in node.children().filter(|n| n.is_element()){
            match ch.xsd_type()? {
                ElementType::AppInfo => {res.app_infos.push(AppInfo::parse(ch)?)}
                ElementType::Documentation => {res.documentations.push(Documentation::parse(ch)?)}
                _ => return Err(format!("Invalid child node for xsd:annotation element: {:?}", node))
            };
        }
        for attr in node.attributes() {
            match attr.name() {
                "id" => {res.id = Some(attr.into())}
                _ => res.attributes.push(attr.clone())
            };
        }

        Ok(res)
    }
}

#[cfg(test)]
mod test {
use crate::xsd_model::elements::annotation::Annotation;
    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
            r#"<annotation id="ID" xml:lang="us" a='a' b='a'>
            <appInfo>Some appinfo</appInfo>
            <appInfo>Some appinfo2</appInfo>
            <documentation>Some doc</documentation>
            <documentation>Some doc2</documentation>
            </annotation>"#
        ).unwrap();
        let root = doc.root_element();
        let res = Annotation::parse(root).unwrap();
        assert_eq!(res.app_infos.len(), 2);
        assert_eq!(res.documentations.len(), 2);
        assert_eq!(res.attributes.len(), 3);
        assert_eq!(res.id.unwrap().0, "ID");
    }
}
