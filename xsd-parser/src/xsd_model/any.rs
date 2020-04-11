use crate::xsd_model::annotation::Annotation;
use crate::xsd_model::{RawAttribute, MaxOccurs};
use crate::xsd_model::xsd::{Id, NonNegativeInteger};

// See http://www.w3.org/TR/xmlschema-1/#element-any.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
// xsd:annotation [0..1]    from type xsd:annotated
//
// Attributes
// Any attribute	[0..*]		                    Namespace: ##other, Process Contents: lax	            from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                                                    from type xsd:annotated
// namespace	    [0..1]	xsd:namespaceList	    Default value is "##any".                               from type xsd:wildcard
// processContents	[0..1]	Anonymous		        Default value is "strict".                              from type xsd:wildcard
// minOccurs	    [0..1]	xsd:nonNegativeInteger	minimum number of occurrences	Default value is "1".   from group xsd:occurs
// maxOccurs	    [0..1]	Anonymous	            maximum number of occurrences	Default value is "1".   from group xsd:occurs
pub struct Any<'a> {
    annotation: Option<Annotation<'a>>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    namespace: &'a str, //TODO: namespaceList
    process_contents: &'a str,
    min_occurs: NonNegativeInteger,
    max_occurs: MaxOccurs,
}