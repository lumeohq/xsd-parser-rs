use crate::xsd_model::complex_types::{local_simple_type, top_level_simple_type};

// xsd:simpleType
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: xsd:localSimpleType
// Properties: Local, Qualified
//
// Used in
// Anonymous type of element xsd:list
// Anonymous type of element xsd:union
// Group xsd:elementModel
// Group xsd:simpleRestrictionModel
// Anonymous type of element xsd:restriction via reference to xsd:simpleRestrictionModel
// Type xsd:localAttributeType (Element xsd:attribute)
// Type xsd:topLevelAttributeType (Element xsd:attribute)
// Type xsd:localElement via reference to xsd:elementModel (Element xsd:element)
// Type xsd:narrowMaxMin via reference to xsd:elementModel (Element xsd:element)
// Type xsd:simpleRestrictionType via reference to xsd:simpleRestrictionModel (Element xsd:restriction)
// Type xsd:topLevelElement via reference to xsd:elementModel (Element xsd:element)
pub type LocalSimpleType<'a> = local_simple_type::LocalSimpleType<'a>;

// xsd:simpleType
// See http://www.w3.org/TR/xmlschema-2/#element-simpleType.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: xsd:topLevelSimpleType
// Properties: Global, Qualified
//
// Used in
// Group xsd:redefinable
// Anonymous type of element xsd:redefine via reference to xsd:redefinable
// Anonymous type of element xsd:schema via reference to xsd:schemaTop
// Group xsd:schemaTop via reference to xsd:redefinable
pub type TopLevelSimpleType<'a> = top_level_simple_type::TopLevelSimpleType<'a>;
