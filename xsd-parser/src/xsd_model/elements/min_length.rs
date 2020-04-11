use crate::xsd_model::facets::NumFacet;

// xsd:minLength
// See http://www.w3.org/TR/xmlschema-2/#element-minLength.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: xsd:numFacet
// Properties: Global, Qualified
//
// Used in
// Group xsd:facets
// Anonymous type of element xsd:restriction via reference to xsd:simpleRestrictionModel
// Group xsd:simpleRestrictionModel via reference to xsd:facets
// Type xsd:simpleRestrictionType via reference to xsd:simpleRestrictionModel (Element xsd:restriction)
pub type MinLength<'a> = NumFacet<'a>;