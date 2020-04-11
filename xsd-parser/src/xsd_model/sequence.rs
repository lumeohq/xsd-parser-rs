use crate::xsd_model::annotation::Annotation;
use crate::xsd_model::common_groups::NestedParticle;
use crate::xsd_model::{RawAttribute, MaxOccurs};
use crate::xsd_model::xsd::{Id, NonNegativeInteger};

// See http://www.w3.org/TR/xmlschema-1/#element-sequence.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:explicitGroup
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]       from type xsd:annotated
//      Choice [0..*]               from group xsd:nestedParticle
//          xsd:element
//          xsd:group
//          xsd:choice
//          xsd:sequence
//          xsd:any
//
// Attributes
// Any attribute	[0..*]		                    Namespace: ##other, Process Contents: lax	            from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                                                    from type xsd:annotated
// minOccurs	    [0..1]	xsd:nonNegativeInteger	minimum number of occurrences	Default value is "1".   from group xsd:occurs
// maxOccurs	    [0..1]	Anonymous	            maximum number of occurrences	Default value is "1".   from group xsd:occurs
pub struct Sequence<'a> {
    annotation: Option<Annotation<'a>>,
    nested_particle: Vec<NestedParticle<'a>>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    min_occurs: NonNegativeInteger,
    max_occurs: MaxOccurs,
}

