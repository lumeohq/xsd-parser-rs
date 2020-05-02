use crate::xml_to_xsd::utils::annotation_first;
use crate::xsd_model::groups::complex_type_model::ComplexTypeModel;
use crate::xsd_model::simple_types::derivation_set::DerivationSet;
use crate::xsd_model::TopLevelComplexType;
use roxmltree::Node;

impl<'a> TopLevelComplexType<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let annotation = annotation_first(node)?;
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
                "name" => name = Some(attr.into()),
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

#[cfg(test)]
mod test {
    use crate::xsd_model::groups::type_def_particle::TypeDefParticle;
    use crate::xsd_model::TopLevelComplexType;

    #[test]
    fn test_top_level_complex_type_parse() {
        let doc = roxmltree::Document::parse(
            r##"
	<complexType name="FloatRange" xmlns="http://www.w3.org/2001/XMLSchema" id="ID" a='a'>
		<annotation>
			<documentation>DocText</documentation>
		</annotation>
		<sequence>
			<element name="Min" type="xs:float"/>
			<element name="Max" type="xs:float"/>
		</sequence>
	</complexType>
                 "##,
        )
        .unwrap();
        let root = doc.root_element();
        let res = TopLevelComplexType::parse(root).unwrap();
        assert_eq!(res.annotation.as_ref().unwrap().doc_str(0), Some("DocText"));
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.id.as_ref().unwrap().0, "ID");
        assert_eq!(res.name.0, "FloatRange");
        if let TypeDefParticle::Sequence(val) = res.type_def_particle().unwrap() {
            assert_eq!(val.nested_particle.len(), 2);
        } else {
            panic!();
        }
    }
}
