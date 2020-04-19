use crate::xml_to_xsd::utils::annotation_first;
use crate::xsd_model::groups::complex_type_model::ComplexTypeModel;
use crate::xsd_model::simple_types::derivation_set::DerivationSet;
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::TopLevelComplexType;
use roxmltree::Node;

impl<'a> TopLevelComplexType<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let annotation = annotation_first(node);
        let model = ComplexTypeModel::parse(node)?;

        let mut attributes = vec![];
        let mut id = None;
        let mut name = None;
        let mut abstract_ = false;
        let mut final_ = None;
        let mut block = None;
        let mut mixed = false;

        for attr in node.attributes() {
            match attr.name() {
                "id" => id = Some(attr.into()),
                "name" => name = Some(QName::new(attr.value())),
                "abstract" => {
                    abstract_ = attr
                        .value()
                        .parse()
                        .map_err(|err| format!("Invalid 'abstract' attribute value: {}", err))?
                }
                "final" => final_ = Some(DerivationSet::parse(attr.value())?),
                "block" => block = Some(DerivationSet::parse(attr.value())?),
                "mixed" => {
                    mixed = attr
                        .value()
                        .parse()
                        .map_err(|err| format!("Invalid 'mixed' attribute value: {}", err))?
                }
                _ => attributes.push(attr.clone()),
            };
        }

        Ok(Self {
            annotation,
            model,
            attributes,
            id,
            name: name
                .ok_or_else(|| format!("Name required for xsd:topLevelComplexType: {:?}", node))?,
            abstract_,
            final_,
            block,
            mixed,
        })
    }
}
