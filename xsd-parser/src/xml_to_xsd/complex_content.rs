use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::{
    Annotation, ComplexContent, ComplexContentChoice, ComplexRestriction, Extension,
};
use roxmltree::Node;

impl<'a> ComplexContent<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut annotation = None;
        let mut content = None;
        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Annotation => annotation = Some(Annotation::parse(ch)?),
                ElementType::Restriction => {
                    content = Some(ComplexContentChoice::Restriction(Box::new(
                        ComplexRestriction::parse(ch)?,
                    )))
                }
                ElementType::Extension => {
                    content = Some(ComplexContentChoice::Extension(Box::new(Extension::parse(
                        ch,
                    )?)))
                }
                _ => {
                    return Err(format!(
                        "Invalid child node for xsd:complexContent element: {:?}",
                        ch
                    ))
                }
            }
        }
        let content = content
            .ok_or_else(|| format!("Content required for xsd:complexType element: {:?}", node))?;

        let mut attributes = vec![];
        let mut id = None;
        let mut mixed = None;

        for attr in node.attributes() {
            match attr.name() {
                "mixed" => {
                    mixed = Some(attr.value().parse().map_err(|_| {
                        format!("Invalid 'mixed' attribute value: {}", attr.value())
                    })?)
                }
                "id" => id = Some(attr.into()),
                _ => attributes.push(attr.clone()),
            };
        }
        Ok(Self {
            annotation,
            content,
            attributes,
            id,
            mixed,
        })
    }
}
