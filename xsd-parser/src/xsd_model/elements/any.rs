use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::simple_types::non_negative_integer::NonNegativeInteger;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::{MaxOccurs, RawAttribute};

// xsd:any
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
//
// Used in
// Group xsd:nestedParticle
// Type xsd:explicitGroup via reference to xsd:nestedParticle (Elements xsd:choice, xsd:sequence)
// Type xsd:simpleExplicitGroup via reference to xsd:nestedParticle (Elements xsd:choice, xsd:sequence)
#[derive(Debug)]
pub struct Any<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub attributes: Vec<RawAttribute<'a>>,
    pub id: Id<'a>,
    pub namespace: &'a str, //TODO: namespaceList
    pub process_contents: &'a str,
    pub min_occurs: NonNegativeInteger,
    pub max_occurs: MaxOccurs,
}
