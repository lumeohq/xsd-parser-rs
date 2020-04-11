use crate::xsd_model::complex_types::facet::Facet;

// xsd:maxExclusive
// See http://www.w3.org/TR/xmlschema-2/#element-maxExclusive.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: xsd:facet
// Properties: Global, Qualified
//
// Used in
// Group xsd:facets
// Anonymous type of element xsd:restriction via reference to xsd:simpleRestrictionModel
// Group xsd:simpleRestrictionModel via reference to xsd:facets
// Type xsd:simpleRestrictionType via reference to xsd:simpleRestrictionModel (Element xsd:restriction)
pub type MaxExclusive<'a> = Facet<'a>;