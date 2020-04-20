use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::{Annotation, SimpleContent, SimpleContentChoice, SimpleExtension};
use roxmltree::Node;

impl<'a> SimpleContent<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut annotation = None;
        let mut content = None;
        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Annotation => annotation = Some(Annotation::parse(ch)?),
                x => content = Some(SimpleContentChoice::parse(ch, x)?),
            }
        }

        let content =
            content.ok_or_else(|| format!("Content required for xsd:simpleContent: {:?}", node))?;
        let mut attributes = vec![];
        let mut id = None;

        for attr in node.attributes() {
            match attr.name() {
                "id" => id = Some(attr.into()),
                _ => attributes.push(attr.clone()),
            };
        }

        Ok(Self {
            annotation,
            content,
            attributes,
            id,
        })
    }
}

impl<'a> SimpleContentChoice<'a> {
    pub fn parse(node: Node<'a, '_>, element_type: ElementType) -> Result<Self, String> {
        Ok(match element_type {
            ElementType::Extension => {
                SimpleContentChoice::Extension(Box::new(SimpleExtension::parse(node)?))
            }
            //ElementType::Restriction => SimpleContentChoice::Restriction(Box::new(SimpleRestriction::parse(node)?)),
            _ => {
                return Err(format!(
                    "Invalid node for xsd:simpleContent content: {:?}",
                    node
                ))
            }
        })
    }
}
