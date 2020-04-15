use roxmltree::Node;

use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::complex_types::facet::Facet;
use crate::xsd_model::complex_types::no_fixed_facet::NoFixedFacet;
use crate::xsd_model::complex_types::num_facet::NumFacet;
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::groups::facets::Facets;
use crate::xsd_model::{Pattern, TotalDigits, WhiteSpace};
use std::str::ParseBoolError;
use crate::xml_to_xsd::utils::annotation_only;

impl<'a> Facet<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        res.annotation = annotation_only(node, "facet")?;

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
        res.annotation = annotation_only(node, "totalDigits")?;

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
        res.annotation = annotation_only(node, "numFacet")?;

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
        res.annotation = annotation_only(node, "noFixedFacet")?;

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
        let annotation = annotation_only(node, "whiteSpace")?;

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

        let value =
            value.ok_or_else(|| "value attribute required for xsd:whiteSpace".to_string())?;

        Ok(Self {
            annotation,
            id,
            value,
            fixed,
            attributes,
        })
    }
}

impl<'a> Pattern<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        res.annotation = annotation_only(node, "whiteSpace")?;

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

impl<'a> Facets<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let res = match node.xsd_type()? {
            ElementType::MinExclusive => Facets::MinExclusive(Facet::parse(node)?),
            ElementType::MinInclusive => Facets::MinInclusive(Facet::parse(node)?),
            ElementType::MaxExclusive => Facets::MaxExclusive(Facet::parse(node)?),
            ElementType::MaxInclusive => Facets::MaxInclusive(Facet::parse(node)?),
            ElementType::TotalDigits => Facets::TotalDigits(TotalDigits::parse(node)?),
            ElementType::FractionDigits => Facets::FractionDigits(NumFacet::parse(node)?),
            ElementType::Length => Facets::Length(NumFacet::parse(node)?),
            ElementType::MinLength => Facets::MinLength(NumFacet::parse(node)?),
            ElementType::MaxLength => Facets::MaxLength(NumFacet::parse(node)?),
            ElementType::Enumeration => Facets::Enumeration(NoFixedFacet::parse(node)?),
            ElementType::WhiteSpace => Facets::WhiteSpace(WhiteSpace::parse(node)?),
            ElementType::Pattern => Facets::Pattern(Pattern::parse(node)?),
            _ => return Err(format!("Error facet parsing. Invalid node: {:?}", node)),
        };

        Ok(res)
    }
}
