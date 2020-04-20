use crate::xml_to_xsd::utils::annotation_first;
use crate::xsd_model::groups::complex_type_model::ComplexTypeModel;
use crate::xsd_model::LocalComplexType;
use roxmltree::Node;

impl<'a> LocalComplexType<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self {
            annotation: annotation_first(node)?,
            model: ComplexTypeModel::parse(node)?,
            id: None,
            mixed: false,
            attributes: vec![],
        };

        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.into()),
                "mixed" => {
                    res.mixed = attr
                        .value()
                        .parse()
                        .map_err(|err| format!("Invalid 'mixed' attribute value: {}", err))?
                }
                _ => res.attributes.push(attr.clone()),
            };
        }

        Ok(res)
    }
}
