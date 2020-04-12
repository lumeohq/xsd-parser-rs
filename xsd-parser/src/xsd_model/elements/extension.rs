use crate::xsd_model::complex_types::extension_type::ExtensionType;
use crate::xsd_model::complex_types::simple_extension_type::SimpleExtensionType;

// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:simpleExtensionType
// Properties: Local, Qualified
//
// Used in
// Anonymous type of element xsd:simpleContent
pub type SimpleExtension<'a> = SimpleExtensionType<'a>;

// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:extensionType
// Properties: Local, Qualified
//
// Attributes
// Any attribute	[0..*]		Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	[0..1]	xsd:ID		from type xsd:annotated
// base	[1..1]	xsd:QName
//
// Used in
// Anonymous type of element xsd:complexContent
pub type Extension<'a> = ExtensionType<'a>;
