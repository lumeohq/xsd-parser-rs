#[cfg(test)]
mod test {
    use crate::xml_to_xsd::schema::parse_document;
    use crate::xsd_model::simple_types::form_choice::FormChoice;
    use crate::xsd_model::Schema;
    use roxmltree::Document;

    const TEXT: &str = include_str!("fixtures/schema.xsd");
    #[test]
    fn test_parse_document() {
        let doc = Document::parse(TEXT).unwrap();
        let schema = parse_document(&doc).unwrap();
        assert_eq!(
            schema.target_namespace.as_ref().unwrap().0,
            "http://www.onvif.org/ver10/schema"
        );

        test_attributes(&schema);
        test_includes(&schema);
        test_imports(&schema);
    }

    fn test_attributes(schema: &Schema) {
        assert_eq!(
            schema.target_namespace.as_ref().unwrap().0,
            "http://www.onvif.org/ver10/schema"
        );
        assert_eq!(schema.element_form_default, FormChoice::Qualified);
        assert_eq!(schema.version.as_ref().unwrap().0, "1.0");
        assert_eq!(schema.id.as_ref().unwrap().0, "ID");
        assert_eq!(schema.attributes.len(), 3);
        assert_eq!(schema.attributes[2].value(), "C");
    }

    fn test_includes(schema: &Schema) {
        assert_eq!(schema.includes.len(), 4);
        assert_eq!(schema.includes[0].schema_location.0, "common1.xsd");
        assert_eq!(schema.includes[1].schema_location.0, "common2.xsd");
        assert_eq!(schema.includes[2].schema_location.0, "common3.xsd");
        assert_eq!(schema.includes[3].schema_location.0, "common4.xsd");
    }

    fn test_imports(schema: &Schema) {
        assert_eq!(schema.imports.len(), 4);
        assert_eq!(
            schema.imports[1].namespace.as_ref().unwrap().0,
            "http://www.w3.org/2003/05/soap-envelope"
        );
        assert_eq!(
            schema.imports[3].namespace.as_ref().unwrap().0,
            "http://www.w3.org/2004/08/xop/include"
        );
        assert_eq!(
            schema.imports[0].schema_location.as_ref().unwrap().0,
            "http://www.w3.org/2005/05/xmlmime"
        );
        assert_eq!(
            schema.imports[2].schema_location.as_ref().unwrap().0,
            "http://docs.oasis-open.org/wsn/b-2.xsd"
        );
    }
}
