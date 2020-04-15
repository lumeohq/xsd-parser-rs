use roxmltree::Node;

use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::complex_types::facet::Facet;
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::{Annotation, TotalDigits, WhiteSpace, Pattern};
use std::str::ParseBoolError;
use crate::xsd_model::complex_types::num_facet::NumFacet;
use crate::xsd_model::complex_types::no_fixed_facet::NoFixedFacet;

impl<'a> Facet<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();

        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Annotation => res.annotation = Some(Annotation::parse(ch)?),
                _ => return Err(format!("Invalid child node for xsd:facet type: {:?}", node)),
            };
        }
        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.into()),
                "value" => res.value = attr.value(),
                "fixed" => {
                    res.fixed = attr
                        .value()
                        .parse()
                        .map_err(|er: ParseBoolError| er.to_string())?
                }
                _ => res.attributes.push(attr.clone()),
            };
        }

        Ok(res)
    }
}

impl<'a> TotalDigits<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();

        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Annotation => res.annotation = Some(Annotation::parse(ch)?),
                _ => return Err(format!("Invalid child node for xsd:facet type: {:?}", node)),
            };
        }
        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.into()),
                "value" => res.value = attr.value().parse()?,
                "fixed" => {
                    res.fixed = attr
                        .value()
                        .parse()
                        .map_err(|er: ParseBoolError| er.to_string())?
                }
                _ => res.attributes.push(attr.clone()),
            };
        }

        Ok(res)
    }
}

impl<'a> NumFacet<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();

        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Annotation => res.annotation = Some(Annotation::parse(ch)?),
                _ => return Err(format!("Invalid child node for xsd:NumFacet type: {:?}", node)),
            };
        }
        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.into()),
                "value" => res.value = attr.value().parse()?,
                "fixed" => {
                    res.fixed = attr
                        .value()
                        .parse()
                        .map_err(|er: ParseBoolError| er.to_string())?
                }
                _ => res.attributes.push(attr.clone()),
            };
        }

        Ok(res)
    }
}


impl<'a> NoFixedFacet<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();

        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Annotation => res.annotation = Some(Annotation::parse(ch)?),
                _ => return Err(format!("Invalid child node for xsd:NoFixedFacet type: {:?}", node)),
            };
        }
        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.into()),
                "value" => res.value = attr.value(),
                _ => res.attributes.push(attr.clone()),
            };
        }

        Ok(res)
    }
}


impl<'a> WhiteSpace<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut annotation = None;

        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Annotation =>annotation = Some(Annotation::parse(ch)?),
                _ => return Err(format!("Invalid child node for xsd:whiteSpace type: {:?}", node)),
            };
        }

        let mut id = None;
        let mut value = None;
        let mut fixed = false;
        let mut attributes = vec![];
        for attr in node.attributes() {
            match attr.name() {
                "id" => id = Some(attr.into()),
                "value" => value = Some(attr.value().parse()?),
                "fixed" => {
                    fixed = attr
                        .value()
                        .parse()
                        .map_err(|er: ParseBoolError| er.to_string())?
                }
                _ => attributes.push(attr.clone()),
            };
        }

        let value = value.ok_or( "value attribute required for xsd:whiteSpace".to_string())?;

        Ok(Self{
            annotation,
            id,
            value,
            fixed,
            attributes
        })
    }
}


impl<'a> Pattern<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();

        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Annotation => res.annotation = Some(Annotation::parse(ch)?),
                _ => return Err(format!("Invalid child node for xsd:NoFixedFacet type: {:?}", node)),
            };
        }
        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.into()),
                "value" => res.value = attr.value(),
                _ => res.attributes.push(attr.clone()),
            };
        }

        Ok(res)
    }
}