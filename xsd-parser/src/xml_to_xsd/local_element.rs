use crate::xml_to_xsd::utils::annotation_first;
use crate::xml_to_xsd::{ElementChildren_, XsdNode};
use crate::xsd_model::groups::element_model::ElementModel;
use crate::xsd_model::simple_types::block_set::BlockSet;
use crate::xsd_model::simple_types::form_choice::FormChoice;
use crate::xsd_model::simple_types::non_negative_integer::NonNegativeInteger;
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::{LocalElement, MaxOccurs};
use roxmltree::Node;

impl<'a> LocalElement<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self {
            annotation: annotation_first(node)?,
            model: ElementModel::parse(node)?,
            ..Default::default()
        };

        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.into()),
                "name" => res.name = Some(attr.into()),
                "ref" => res.ref_ = Some(QName::new(attr.value())),
                "type" => res.type_ = Some(QName::new(attr.value())),
                "minOccurs" => res.min_occurs = attr.value().parse()?,
                "maxOccurs" => res.max_occurs = attr.value().parse()?,
                "default" => res.default = Some(attr.value()),
                "fixed" => res.fixed = Some(attr.value()),
                "nillable" => {
                    res.nillable = attr.value().parse().map_err(|_| {
                        format!("Invalid 'nillable' attribute value: {}", attr.value())
                    })?
                }
                "block" => res.block = Some(BlockSet::parse(attr.value())?),
                "form" => res.form = Some(FormChoice::parse(attr.value())?),

                _ => res.attributes.push(attr.clone()),
            };
        }

        Ok(res)
    }
}
