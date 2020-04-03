#[cfg(test)]
mod test {
    use crate::parser::types::TypeModifier;

    #[test]
    fn test_extension_base() {
        use crate::parser::parse;
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

    #[test]
    fn test_restriction_any_type() {
        use crate::parser::parse;
        use crate::parser::types::RsEntity;

        let text = r#"
<xs:schema targetNamespace="http://schemas.xmlsoap.org/ws/2005/04/discovery"
    xmlns:tns="http://schemas.xmlsoap.org/ws/2005/04/discovery"
    xmlns:wsa="http://schemas.xmlsoap.org/ws/2004/08/addressing"
    xmlns:xs="http://www.w3.org/2001/XMLSchema"
    elementFormDefault="qualified" blockDefault="\#all">

    <xs:element name="AppSequence" type="tns:AppSequenceType"/>
    <xs:complexType name="AppSequenceType">
        <xs:complexContent>
            <xs:restriction base="xs:anyType">
                <xs:attribute name="InstanceId" type="xs:unsignedInt" use="required"/>
                <xs:attribute name="SequenceId" type="xs:anyURI"/>
                <xs:attribute name="MessageNumber" type="xs:unsignedInt" use="required"/>
                <xs:anyAttribute namespace="\#\#other" processContents="lax"/>
            </xs:restriction>
        </xs:complexContent>
    </xs:complexType>

</xs:schema>
        "#;

        let result = parse(text).unwrap();
        println!("{:?}", result);
        assert_eq!(result.types.len(), 2);

        match &result.types[0] {
            RsEntity::Alias(s) => {
                assert_eq!(s.name, "AppSequence");
                assert_eq!(s.original, "tns:AppSequenceType");
            }
            _ => unreachable!(),
        }

        match &result.types[1] {
            RsEntity::Struct(s) => {
                assert_eq!(s.fields.borrow().len(), 4);

                assert_eq!(s.fields.borrow()[0].name, "InstanceId");
                assert_eq!(s.fields.borrow()[0].type_name, "xs:unsignedInt");

                assert_eq!(s.fields.borrow()[1].name, "SequenceId");
                assert_eq!(s.fields.borrow()[1].type_name, "xs:anyURI");
                assert_eq!(s.fields.borrow()[1].type_modifiers[0], TypeModifier::Option);

                assert_eq!(s.fields.borrow()[2].name, "MessageNumber");
                assert_eq!(s.fields.borrow()[2].type_name, "xs:unsignedInt");

                assert_eq!(s.fields.borrow()[3].name, "any_attribute");

                assert_eq!(s.name, "AppSequenceType");
            }
            _ => unreachable!(),
        }
    }
}
