use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::RawAttribute;

// xsd:wildcard
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
//
// Content
//  xsd:annotation [0..1]   from type xsd:annotated
//
// Attributes
// Any attribute	[0..*]		            Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                                from type xsd:annotated
// namespace	    [0..1]	xsd:namespaceList		    Default value is "##any".
// processContents	[0..1]	Anonymous		            Default value is "strict".
//
// Used by
// Element xsd:anyAttribute
// Element xsd:any via derived anonymous type
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:wildcard
//                  extended by Anonymous type of element xsd:any
pub struct Wildcard<'a> {
    annotation: Option<Annotation<'a>>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    namespace: &'a str, //TODO: namespaceList
    process_contents: &'a str,
}
