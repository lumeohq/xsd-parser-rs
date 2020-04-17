use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::complex_types::local_attribute_type::UseType;
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::simple_types::form_choice::FormChoice;
use crate::xsd_model::simple_types::ncname::NCName;
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::{Annotation, LocalAttribute, LocalSimpleType, TopLevelAttribute};
use roxmltree::Node;

impl<'a> TopLevelAttribute<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut annotation = None;
        let mut simple_type = None;
        let mut attributes = vec![];
        let mut id = None;
        let mut name: Option<NCName> = None;
        let mut type_ = None;
        let mut default = None;
        let mut fixed = None;

        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Annotation => annotation = Some(Annotation::parse(ch)?),
                ElementType::SimpleType => simple_type = Some(LocalSimpleType::parse(ch)?),
                _ => {
                    return Err(format!(
                        "Invalid child type of xsd:topLevelAttribute: {:?}",
                        node
                    ))
                }
            }
        }

        for attr in node.attributes() {
            match attr.name() {
                "id" => id = Some(attr.into()),
                "name" => name = Some(attr.into()),
                "type" => type_ = Some(QName::new(attr.value())),
                "default" => default = Some(attr.value()),
                "fixed" => fixed = Some(attr.value()),
                _ => attributes.push(attr.clone()),
            };
        }

        let name =
            name.ok_or_else(|| format!("Name required for xsd:topLevelAttribute: {:?}", node))?;

        Ok(Self {
            annotation,
            simple_type,
            attributes,
            id,
            name,
            type_,
            default,
            fixed,
        })
    }
}

impl<'a> LocalAttribute<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut annotation = None;
        let mut simple_type = None;
        let mut attributes = vec![];
        let mut id = None;
        let mut name: Option<NCName> = None;
        let mut ref_: Option<QName> = None;
        let mut type_: Option<QName> = None;
        let mut use_ = UseType::Optional;
        let mut default = None;
        let mut fixed = None;
        let mut form = None;

        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Annotation => annotation = Some(Annotation::parse(ch)?),
                ElementType::SimpleType => simple_type = Some(LocalSimpleType::parse(ch)?),
                _ => {
                    return Err(format!(
                        "Invalid child type of xsd:localAttribute: {:?}",
                        node
                    ))
                }
            }
        }

        for attr in node.attributes() {
            match attr.name() {
                "id" => id = Some(attr.into()),
                "name" => name = Some(attr.into()),
                "ref" => ref_ = Some(QName::new(attr.value())),
                "type" => type_ = Some(QName::new(attr.value())),
                "use_" => use_ = UseType::parse(attr.value())?,
                "default" => default = Some(attr.value()),
                "fixed" => fixed = Some(attr.value()),
                "form" => form = Some(FormChoice::parse(attr.value())?),
                _ => attributes.push(attr.clone()),
            };
        }

        let name =
            name.ok_or_else(|| format!("Name required for xsd:topLevelAttribute: {:?}", node))?;

        Ok(Self {
            annotation,
            simple_type,
            attributes,
            id,
            name,
            ref_,
            type_,
            use_,
            default,
            fixed,
            form,
        })
    }
}
