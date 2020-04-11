use crate::xsd_model::datatypes::complex_types::num_facet::NumFacet;

// xsd:length
// See http://www.w3.org/TR/xmlschema-2/#element-length.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: xsd:numFacet
// Properties: Global, Qualified
//
// Content
// xsd:annotation [0..1]
//
// Attributes
// id	            [0..1]	xsd:ID		                                        from type xsd:annotated
// fixed	        [0..1]	xsd:boolean		        Default value is "false".   from type xsd:facet
// value	        [1..1]	xsd:nonNegativeInteger
// Any attribute	[0..*]		                    Namespace: ##other, Process Contents: lax
//
// Used in
// Group xsd:facets
// Anonymous type of element xsd:restriction via reference to xsd:simpleRestrictionModel
// Group xsd:simpleRestrictionModel via reference to xsd:facets
// Type xsd:simpleRestrictionType via reference to xsd:simpleRestrictionModel (Element xsd:restriction)
pub type Length<'a> = NumFacet<'a>;