use crate::xsd_model::complex_types::local_attribute_type::LocalAttributeType;
use crate::xsd_model::complex_types::top_level_attribute_type::TopLevelAttributeType;

// xsd:attribute
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:localAttributeType
// Properties: Local, Qualified
//
// Used in
// Group xsd:attrDecls
// Group xsd:complexTypeModel via reference to xsd:attrDecls
// Type xsd:complexType via reference to xsd:complexTypeModel
// Type xsd:complexRestrictionType via reference to xsd:attrDecls (Element xsd:restriction)
// Type xsd:extensionType via reference to xsd:attrDecls (Element xsd:extension)
// Type xsd:localComplexType via reference to xsd:complexTypeModel (Element xsd:complexType)
// Type xsd:namedAttributeGroup via reference to xsd:attrDecls (Element xsd:attributeGroup)
// Type xsd:simpleExtensionType via reference to xsd:attrDecls (Element xsd:extension)
// Type xsd:simpleRestrictionType via reference to xsd:attrDecls (Element xsd:restriction)
// Type xsd:topLevelComplexType via reference to xsd:complexTypeModel (Element xsd:complexType)
pub type LocalAttribute<'a> = LocalAttributeType<'a>;

// xsd:attribute
// See http://www.w3.org/TR/xmlschema-1/#element-attribute.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:topLevelAttributeType
// Properties: Global, Qualified
//
// Used in
// Group xsd:schemaTop
// Anonymous type of element xsd:schema via reference to xsd:schemaTop
pub type TopLevelAttribute<'a> = TopLevelAttributeType<'a>;
