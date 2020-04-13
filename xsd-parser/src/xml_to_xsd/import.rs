use roxmltree::Node;
use crate::xsd_model::Import;
use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::Annotation;

impl<'a> Import<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Import<'a>, String> {
        let mut res = Import::default();
        if let Some(child) = node.first_element_child() {
            if child.xsd_type()? != ElementType::Annotation {
                return Err(format!("Invalid content type of xsd:import: {:?}", node))
            } else {
                res.annotation = Some(Annotation::parse(child)?);
            }
        }

        for attr in node.attributes() {
            match attr.name() {
                "schemaLocation" => {res.schema_location = Some(attr.into())}
                "id" => {res.id = Some(attr.into())}
                "namespace" => {res.namespace = Some(attr.into())}
                _ => res.attributes.push(attr.clone())
            };
        }

        Ok(res)
    }
}


#[cfg(test)]
mod test {
use crate::xsd_model::Import;
    #[test]
    fn test_parse() {
        let doc = roxmltree::Document::parse(
            r#"<import id="ID" namespace="xsd" schemaLocation="http://uri" b='a'>
                    <annotation>
                        <appInfo>Some appinfo</appInfo>
                        <documentation>Some doc2</documentation>
                    </annotation>
            </import>"#
        ).unwrap();
        let root = doc.root_element();
        let res = Import::parse(root).unwrap();
        assert_eq!(res.annotation.as_ref().unwrap().documentations.len(), 1);
        assert_eq!(res.annotation.unwrap().app_infos.len(), 1);
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.id.unwrap().0, "ID");
        assert_eq!(res.schema_location.unwrap().0, "http://uri");
        assert_eq!(res.namespace.unwrap().0, "xsd");
    }
}