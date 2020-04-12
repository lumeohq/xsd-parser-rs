use roxmltree::Node;
use crate::xsd_model::List;
use crate::xsd_model::elements::ElementType;
use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::Annotation;
use crate::xsd_model::LocalSimpleType;
use crate::xsd_model::simple_types::qname::QName;

impl<'a> List<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<List<'a>, String> {
        let mut res = List::default();

        for ch in node.children().filter(|n| n.is_element()){
            match ch.xsd_type()? {
                ElementType::Annotation => {res.annotation = Some(Annotation::parse(ch)?)}
                ElementType::SimpleType => {res.simple_type = Some(LocalSimpleType::parse(ch)?)}
                _ => Err(format!("Invalid child node for xsd:list element: {:?}", node))?
            };
        }
        for attr in node.attributes() {
            match attr.name() {
                "id" => {res.id = Some(attr.into())}
                "itemType" => {res.item_type = Some(QName::new(attr.value()))}
                _ => res.attributes.push(attr.clone())
            };
        }

        Ok(res)
    }
}