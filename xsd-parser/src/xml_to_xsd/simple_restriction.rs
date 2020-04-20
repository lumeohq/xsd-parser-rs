use crate::xml_to_xsd::utils::annotation_first;
use crate::xml_to_xsd::{ElementChildren_, XsdNode};
use crate::xsd_model::complex_types::facet::Facet;
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::groups::attr_decls::AttrDecls;
use crate::xsd_model::groups::simple_restriction_model::SimpleRestrictionModel;
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::SimpleRestriction;
use roxmltree::{Children, Node};

impl<'a> SimpleRestriction<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self::default();
        res.annotation = annotation_first(node)?;
        let mut iter = node.element_children();
        res.model = SimpleRestrictionModel::parse(&mut iter)?;
        res.attr_decls = AttrDecls::parse(iter)?;

        let mut base = None;

        for attr in node.attributes() {
            match attr.name() {
                "base" => base = Some(QName::new(attr.value())),
                "id" => res.id = Some(attr.into()),
                _ => res.attributes.push(attr.clone()),
            };
        }

        res.base = base.ok_or_else(|| {
            format!(
                "Attribute 'base' required for xsd:simpleExtension element: {:?}",
                node
            )
        })?;
        Ok(res)
    }
}

fn parse_facets<'a>(
    iter: &'a mut Children<'a, '_>,
    facets: &mut Vec<Facet<'a>>,
) -> Result<&'a Children<'a, 'a>, String> {
    for ch in iter.filter(|n| n.is_element()) {
        match ch.xsd_type()? {
            ElementType::MinExclusive => facets.push(Facet::parse(ch)?),
            ElementType::MinInclusive => facets.push(Facet::parse(ch)?),
            ElementType::MaxExclusive => facets.push(Facet::parse(ch)?),
            ElementType::MaxInclusive => facets.push(Facet::parse(ch)?),
            _ => break,
        };
    }
    Ok(iter)
}
