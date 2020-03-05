#[cfg(test)]
mod test {
    use crate::parser::types::TypeModifier;

    #[test]
    fn test_extension_base() {
        use crate::parser::schema_parser::parse;
        use crate::parser::types::RsEntity;

        let text = r#"
<xs:schema xmlns:tt="http://www.onvif.org/ver10/schema"
    xmlns:xs="http://www.w3.org/2001/XMLSchema"
    targetNamespace="http://www.onvif.org/ver10/schema">

    <xs:complexType name="DeviceEntity">
        <xs:attribute name="token" type="tt:ReferenceToken" use="required"/>
    </xs:complexType>

    <xs:complexType name="VideoSource">
        <xs:complexContent>
            <xs:extension base="tt:DeviceEntity">
                <xs:sequence>
                    <xs:element name="Resolution" type="tt:VideoResolution">
                    </xs:element>
                    <xs:element name="Imaging" type="tt:ImagingSettings" minOccurs="0"/>
                </xs:sequence>
            </xs:extension>
        </xs:complexContent>
    </xs:complexType>

</xs:schema>
        "#;

        let result = parse(text).unwrap();
        assert_eq!(result.types.len(), 2);

        match &result.types[0] {
            RsEntity::Struct(s) => {
                assert_eq!(s.fields.borrow()[0].name, "token");
                assert_eq!(s.fields.borrow()[0].type_name, "tt:ReferenceToken");
                assert_eq!(s.name, "DeviceEntity");
            }
            _ => unreachable!(),
        }

        match &result.types[1] {
            RsEntity::Struct(s) => {
                assert_eq!(s.fields.borrow().len(), 3);

                assert_eq!(s.fields.borrow()[0].name, "Resolution");
                assert_eq!(s.fields.borrow()[0].type_name, "tt:VideoResolution");

                assert_eq!(s.fields.borrow()[1].name, "Imaging");
                assert_eq!(s.fields.borrow()[1].type_name, "tt:ImagingSettings");
                assert_eq!(s.fields.borrow()[1].type_modifiers[0], TypeModifier::Option);

                assert_eq!(s.fields.borrow()[2].name, "token");
                assert_eq!(s.fields.borrow()[2].type_name, "tt:ReferenceToken");
                assert_eq!(s.fields.borrow()[2].type_modifiers[0], TypeModifier::None);

                assert_eq!(s.name, "VideoSource");
            }
            _ => unreachable!(),
        }
    }
}
