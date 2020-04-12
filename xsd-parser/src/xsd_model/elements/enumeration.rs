use crate::xsd_model::complex_types::no_fixed_facet::NoFixedFacet;

// xsd:enumeration
// See http://www.w3.org/TR/xmlschema-2/#element-enumeration.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: xsd:noFixedFacet
// Properties: Global, Qualified
//
// Content
// xsd:annotation [0..1]
//
// Attributes
// id	            [0..1]	xsd:ID		from type xsd:annotated
// value	        [1..1]	xsd:anySimpleType		from type xsd:facet
// Any attribute	[0..*]		Namespace: ##other, Process Contents: lax
//
// Used in
// Group xsd:facets
// Anonymous type of element xsd:restriction via reference to xsd:simpleRestrictionModel
// Group xsd:simpleRestrictionModel via reference to xsd:facets
// Type xsd:simpleRestrictionType via reference to xsd:simpleRestrictionModel (Element xsd:restriction)
pub type Enumeration<'a> = NoFixedFacet<'a>;
