use crate::parser::definitions::Definitions;
use roxmltree::{Document, Node};

pub mod binding;
mod constants;
pub mod definitions;
pub mod message;
pub mod port_type;
pub mod types;


pub fn parse(text: &str) {
    let doc = Document::parse(text).unwrap();
    let definitions = Definitions::new(&doc.root_element());
    println!("{:#?}", definitions);
}

pub trait WsdlElement {
    fn wsdl_type(&self) -> ElementType;
}

#[derive(Debug, PartialEq)]
pub enum ElementType {
    Binding,
    Definitions,
    Documentation,
    Import,
    Input,
    Fault,
    Message,
    Operation,
    Output,
    Part,
    PortType,
    Types,
    UnknownElement(String),
}

impl<'a> WsdlElement for roxmltree::Node<'a, '_> {
    fn wsdl_type(&self) -> ElementType {
        use ElementType::*;
        // TODO: check for wsdl prefix
        match self.tag_name().name() {
            "binding" => Binding,
            "definitions" => Definitions,
            "documentation" => Documentation,
            "import" => Import,
            "input" => Input,
            "fault" => Fault,
            "message" => Message,
            "operation" => Operation,
            "output" => Output,
            "part" => Part,
            "portType" => PortType,
            "types" => Types,
            _ => UnknownElement(self.tag_name().name().to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::definitions::Definitions;
    use roxmltree::Document;

    const TEXT: &str = r#"
<wsdl:definitions
    xmlns:wsdl="http://schemas.xmlsoap.org/wsdl/"
    xmlns:soap="http://schemas.xmlsoap.org/wsdl/soap12/"
    xmlns:xs="http://www.w3.org/2001/XMLSchema"
    xmlns:tds="http://www.onvif.org/ver10/device/wsdl"
    targetNamespace="http://www.onvif.org/ver10/device/wsdl">
	<wsdl:types>
		<xs:schema
            targetNamespace="http://www.onvif.org/ver10/device/wsdl"
            xmlns:tt="http://www.onvif.org/ver10/schema"
            xmlns:tds="http://www.onvif.org/ver10/device/wsdl"
            elementFormDefault="qualified" version="18.12"
		>
			<xs:import namespace="http://www.onvif.org/ver10/schema" schemaLocation="../../../ver10/schema/onvif.xsd"/>
		</xs:schema>
	</wsdl:types>

	<wsdl:message name="GetServicesRequest">
		<wsdl:part name="parameters" element="tds:GetServices"/>
	</wsdl:message>
	<wsdl:message name="GetServicesResponse">
		<wsdl:part name="parameters" element="tds:GetServicesResponse"/>
	</wsdl:message>
	<wsdl:message name="DeleteGeoLocationRequest">
		<wsdl:part name="parameters" element="tds:DeleteGeoLocation"/>
	</wsdl:message>
	<wsdl:message name="DeleteGeoLocationResponse">
		<wsdl:part name="parameters" element="tds:DeleteGeoLocationResponse"/>
	</wsdl:message>

	<wsdl:portType name="Device">
		<wsdl:operation name="GetServices">
			<wsdl:documentation>Returns information about services on the device.</wsdl:documentation>
			<wsdl:input message="tds:GetServicesRequest"/>
			<wsdl:output message="tds:GetServicesResponse"/>
		</wsdl:operation>
		<wsdl:operation name="DeleteGeoLocation">
			<wsdl:documentation>
				This operation deletes the given geo location entries.
			</wsdl:documentation>
			<wsdl:input message="tds:DeleteGeoLocationRequest"/>
			<wsdl:output message="tds:DeleteGeoLocationResponse"/>
		</wsdl:operation>
	</wsdl:portType>

	<wsdl:binding name="DeviceBinding" type="tds:Device">
		<soap:binding style="document" transport="http://schemas.xmlsoap.org/soap/http"/>
		<wsdl:operation name="GetServices">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/GetServices"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
		<wsdl:operation name="DeleteGeoLocation">
			<soap:operation soapAction="http://www.onvif.org/ver10/device/wsdl/DeleteGeoLocation"/>
			<wsdl:input>
				<soap:body use="literal"/>
			</wsdl:input>
			<wsdl:output>
				<soap:body use="literal"/>
			</wsdl:output>
		</wsdl:operation>
	</wsdl:binding>
</wsdl:definitions>
    "#;

    #[test]
    fn test_parse() {
        let doc = Document::parse(TEXT).unwrap();
        let def = Definitions::new(&doc.root_element());

        assert_eq!(
            def.target_namespace().unwrap().uri(),
            "http://www.onvif.org/ver10/device/wsdl"
        );
        assert_eq!(def.types().len(), 1);

        let messages = def.messages();
        assert_eq!(messages.len(), 4);

        for (key, value) in messages {
            assert_eq!(key, &value.name());
            assert_eq!(value.parts().len(), 1);
        }
        assert_eq!(
            def.messages()
                .get("GetServicesRequest")
                .unwrap()
                .parts()
                .len(),
            1
        );
        assert_eq!(def.imports().len(), 0);
        assert_eq!(def.port_types().len(), 1);
    }
}
