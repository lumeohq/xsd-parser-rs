// xsd:simpleType
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Properties: Abstract
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]               from type xsd:annotated
//      Choice [1..1]                   from group xsd:simpleDerivation
//          xsd:restriction
//          xsd:list
//          xsd:union
//
// Attributes
// Any attribute	[0..*]		            Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		from type xsd:annotated
// final	        [0..1]	xsd:simpleDerivationSet
// name	             [0..1]	xsd:NCName	Can be restricted to required or forbidden
//
// Used by
// Element xsd:simpleType via derived type xsd:localSimpleType
// Element xsd:simpleType via derived type xsd:topLevelSimpleType
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:simpleType
//                  restricted by xsd:topLevelSimpleType
//                  restricted by xsd:localSimpleType
pub struct SimpleType {
    // pure abstract type
}