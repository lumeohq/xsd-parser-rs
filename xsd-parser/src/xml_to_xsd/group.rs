use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::complex_types::named_group::{ContentChoice, NamedGroup};
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::simple_types::ncname::NCName;
use crate::xsd_model::Annotation;
use roxmltree::Node;

impl<'a> NamedGroup<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut annotation = None;
        let mut content_choice = None;
        let mut attributes = vec![];
        let mut id = None;
        let mut name: Option<NCName> = None;

        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Annotation => annotation = Some(Annotation::parse(ch)?),
                ty => content_choice = Some(ContentChoice::parse(ch, ty)?),
            }
        }

        let content_choice = content_choice
            .ok_or_else(|| format!("Content required for xsd:namedGroup type: {:?}", node))?;

        for attr in node.attributes() {
            match attr.name() {
                "id" => id = Some(attr.into()),
                "name" => name = Some(attr.into()),
                _ => attributes.push(attr.clone()),
            };
        }

        let name =
            name.ok_or_else(|| format!("Name required for xsd:namedGroup type: {:?}", node))?;

        Ok(Self {
            annotation,
            content_choice,
            attributes,
            id,
            name,
        })
    }
}

impl<'a> ContentChoice<'a> {
    pub fn parse(node: Node<'a, '_>, xsd_type: ElementType) -> Result<Self, String> {
        match xsd_type {
            ElementType::All => unimplemented!("Not present in ONVIF"), //Self::All(All::parse(node)?),
            // ElementType::Choice => Self::Choice(SimpleExplicitGroup::parse(node)?),
            // ElementType::Sequence => Self::Sequence(SimpleExplicitGroup::parse(node)?),
            _ => Err(format!(
                "Invalid content type of xsd:namedGroup: {:?}",
                node
            )),
        }
    }
}
