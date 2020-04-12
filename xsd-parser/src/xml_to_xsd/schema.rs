use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::elements::schema::Schema;
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::simple_types::any_uri::AnyUri;
use crate::xsd_model::simple_types::block_set::BlockSet;
use crate::xsd_model::simple_types::form_choice::FormChoice;
use crate::xsd_model::simple_types::full_derivation_set::FullDerivationSet;
use crate::xsd_model::simple_types::id::Id;
use crate::xsd_model::simple_types::language::Language;
use crate::xsd_model::simple_types::token::Token;
use roxmltree::Document;
use crate::xsd_model::elements::include::Include;
use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::elements::import::Import;

pub fn parse_document<'a>(doc: &'a Document) -> Result<Schema<'a>, String> {
    let schema_node = doc.root_element();
    let mut schema = Schema::default();

    schema.namespaces = Vec::from(schema_node.namespaces());

    for attr in schema_node.attributes() {
        match attr.name() {
            "targetNamespace" => schema.target_namespace = Some(AnyUri(attr.value())),
            "version" => schema.version = Some(Token(attr.value())),
            "finalDefault" => schema.final_default = Some(FullDerivationSet::parse(attr.value())?),
            "blockDefault" => schema.block_default = Some(BlockSet::parse(attr.value())?),
            "attributeFormDefault" => {
                schema.attribute_form_default = FormChoice::parse(attr.value())?
            }
            "elementFormDefault" => schema.element_form_default = FormChoice::parse(attr.value())?,
            "id" => schema.id = Some(Id(attr.value())),
            "lang" => schema.lang = Some(Language(attr.value())),
            _ => schema.attributes.push(attr.clone()),
        };
    }

    for ch in schema_node.children().filter(|n| n.is_element()) {
        match ch.xsd_type()? {
            ElementType::Include => {schema.includes.push(Include::parse(ch)?)}
            ElementType::Import => {schema.imports.push(Import::parse(ch)?)}
            ElementType::Redefine => {}
            ElementType::Annotation => {schema.annotations.push(Annotation::parse(ch)?)}
            //schemaTop
            ElementType::SimpleType => {}
            ElementType::ComplexType => {}
            ElementType::Group => {}
            ElementType::AttributeGroup => {}
            ElementType::Element => {}
            ElementType::Attribute => {}
            ElementType::Notation => {}
            _ => unreachable!("Not TopLevel element! {:#?}", ch),
        }
    }

    Ok(schema)
}

#[cfg(test)]
mod test {
    use super::*;
    use roxmltree::Document;

    const TEXT: &str = r##"
    <xs:schema xmlns:tt="http://www.onvif.org/ver10/schema"
        xmlns:xs="http://www.w3.org/2001/XMLSchema"
        targetNamespace="http://www.onvif.org/ver10/schema"
        version="125"
        finalDefault="#all"
        blockDefault="#all"
        elementFormDefault="qualified"
        id="ID"
        xml:lang="us"
        >
        <xs:include schemaLocation="common.xsd"/>
        <xs:include schemaLocation="common2.xsd"/>
        <xs:import namespace="http://www.w3.org/2005/05/xmlmime" schemaLocation="http://www.w3.org/2005/05/xmlmime"/>
        <xs:import namespace="http://www.w3.org/2003/05/soap-envelope" schemaLocation="http://www.w3.org/2003/05/soap-envelope"/>
        <xs:complexType name="DeviceEntity" />1
        <xs:complexType name="VideoSource" />

    </xs:schema>
            "##;

    #[test]
    fn test_parse_document() {
        let doc = Document::parse(TEXT).unwrap();
        let schema = parse_document(&doc).unwrap();
        assert_eq!(
            schema.target_namespace.unwrap().0,
            "http://www.onvif.org/ver10/schema"
        );
        assert_eq!(schema.version.unwrap().0, "125");
        assert_eq!(schema.final_default.unwrap(), FullDerivationSet::All);
        assert_eq!(schema.block_default.unwrap(), BlockSet::All);
        assert_eq!(schema.id.unwrap().0, "ID");
        assert_eq!(schema.element_form_default, FormChoice::Qualified);
        assert_eq!(schema.lang.unwrap().0, "us");
        assert_eq!(schema.imports.len(), 2);
        assert_eq!(schema.includes.len(), 2);
    }
}
