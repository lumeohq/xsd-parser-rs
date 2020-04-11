use crate::xsd_model::complex_types::local_simple_type::LocalSimpleType;
use crate::xsd_model::groups::facets::Facets;

// xsd:simpleRestrictionModel
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: datatypes.xsd
//
// Content
//  Sequence [1..1]
//      xsd:simpleType [0..1]
//      Choice [0..*]       from group xsd:facets
//          xsd:minExclusive
//          xsd:minInclusive
//          xsd:maxExclusive
//          xsd:maxInclusive
//          xsd:totalDigits
//          xsd:fractionDigits
//          xsd:length
//          xsd:minLength
//          xsd:maxLength
//          xsd:enumeration
//          xsd:whiteSpace
//          xsd:pattern
//
// Used in
// Anonymous type of element xsd:restriction
// Type xsd:simpleRestrictionType (Element xsd:restriction)
pub struct SimpleRestrictionModel<'a> {
    simple_type: Option<LocalSimpleType<'a>>,
    facets: Option<Facets<'a>>
}