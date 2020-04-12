use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::simple_types::AnySimpleType;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::RawAttribute;

// xsd:noFixedFacet
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
//
// Content
// xsd:annotation [0..1]
//
// Attributes
// id	            [0..1]	xsd:ID		from type xsd:annotated
// value	        [1..1]	xsd:anySimpleType		from type xsd:facet
// Any attribute	[0..*]		Namespace: ##other, Process Contents: lax
//
// Used by
// Element xsd:enumeration
// Element xsd:pattern via derived anonymous type
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:facet
//                  xsd:noFixedFacet
//                      restricted by Anonymous type of element xsd:pattern
#[derive(Default, Debug)]
pub struct NoFixedFacet<'a> {
    annotation: Option<Annotation<'a>>,
    id: Id<'a>,
    value: AnySimpleType<'a>,
    attributes: Vec<RawAttribute<'a>>,
}
