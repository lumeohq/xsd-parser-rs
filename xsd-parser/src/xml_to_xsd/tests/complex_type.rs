#[cfg(test)]
mod test {
    use crate::xml_to_xsd::schema::parse_document;
    use crate::xsd_model::groups::nested_particle::NestedParticle;
    use crate::xsd_model::groups::schema_top::SchemaTop;
    use crate::xsd_model::groups::type_def_particle::TypeDefParticle;
    use crate::xsd_model::{Annotation, TopLevelComplexType};
    use roxmltree::Document;
    use std::rc::Rc;

    const TEXT: &str = include_str!("fixtures/complex_types.xsd");
    #[test]
    pub fn test_top_level_complex_type() {
        let doc = Document::parse(TEXT).unwrap();
        let schema = parse_document(&doc).unwrap();
        let mut iter = schema.content.iter();

        test_1(next_complex_type(&mut iter));
    }

    fn next_complex_type<'a>(
        iter: &mut impl Iterator<Item = &'a (SchemaTop<'a>, Option<Annotation<'a>>)>,
    ) -> &'a Rc<TopLevelComplexType<'a>> {
        match &iter.next().unwrap().0 {
            SchemaTop::ComplexType(ct) => ct,
            _ => panic!("Test failed!"),
        }
    }

    fn test_1(ct: &Rc<TopLevelComplexType<'_>>) {
        assert_eq!(ct.name.0, "IntRange");
        assert_eq!(
            ct.annotation.as_ref().unwrap().doc_str(0).unwrap(),
            "Doc Text"
        );

        assert_eq!(ct.attributes.len(), 1);
        assert_eq!(ct.attributes[0].value(), "Whatever!");

        if let TypeDefParticle::Sequence(seq) = ct.type_def_particle().unwrap() {
            assert_eq!(seq.nested_particle.len(), 2);
            if let NestedParticle::Element(el) = &seq.nested_particle[0] {
                assert_eq!(el.name.as_ref().unwrap().0, "Min");
                assert_eq!(el.type_.as_ref().unwrap().name, "int");
                assert_eq!(el.type_.as_ref().unwrap().prefix, Some("xs"));
            } else {
                panic!()
            }

            if let NestedParticle::Element(el) = &seq.nested_particle[1] {
                assert_eq!(el.name.as_ref().unwrap().0, "Max");
                assert_eq!(el.type_.as_ref().unwrap().name, "int");
                assert_eq!(el.type_.as_ref().unwrap().prefix, Some("xs"));
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }
}
