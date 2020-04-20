use crate::xml_to_xsd::utils::annotation_first;
use crate::xsd_model::groups::element_model::ElementModel;
use crate::xsd_model::simple_types::block_set::BlockSet;
use crate::xsd_model::simple_types::derivation_set::DerivationSet;
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::TopLevelElement;
use roxmltree::Node;

// Attributes
// Any attribute	    [0..*]		                Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	                [0..1]	xsd:ID		                                                    from type xsd:annotated
// name	                [1..1]	xsd:NCName
// type	                [0..1]	xsd:QName
// substitutionGroup	[0..1]	xsd:QName
// default	            [0..1]	xsd:string
// fixed	            [0..1]	xsd:string
// nillable	            [0..1]	xsd:boolean		    Default value is "false".
// abstract	            [0..1]	xsd:boolean		    Default value is "false".
// final	            [0..1]	xsd:derivationSet
// block	            [0..1]	xsd:blockSet
impl<'a> TopLevelElement<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut res = Self {
            annotation: annotation_first(node)?,
            model: ElementModel::parse(node)?,
            ..Default::default()
        };

        let mut name = None;

        for attr in node.attributes() {
            match attr.name() {
                "id" => res.id = Some(attr.into()),
                "name" => name = Some(attr.into()),
                "type" => res.type_ = Some(QName::new(attr.value())),
                "substitutionGroup" => res.substitution_group = Some(QName::new(attr.value())),
                "default" => res.default = Some(attr.value()),
                "fixed" => res.fixed = Some(attr.value()),
                "nillable" => {
                    res.nillable = attr.value().parse().map_err(|_| {
                        format!("Invalid 'nillable' attribute value: {}", attr.value())
                    })?
                }
                "abstract" => {
                    res.abstract_ = attr.value().parse().map_err(|_| {
                        format!("Invalid 'nillable' attribute value: {}", attr.value())
                    })?
                }
                "final" => res.final_ = Some(DerivationSet::parse(attr.value())?),
                "block" => res.block = Some(BlockSet::parse(attr.value())?),

                _ => res.attributes.push(attr.clone()),
            };
        }

        res.name =
            name.ok_or_else(|| format!("Name required for xsd:topLevelElement: {:?}", node))?;

        Ok(res)
    }
}
