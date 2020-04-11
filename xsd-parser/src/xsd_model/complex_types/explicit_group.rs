use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::groups::nested_particle::NestedParticle;
use crate::xsd_model::{RawAttribute, MaxOccurs};
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::simple_types::non_negative_integer::NonNegativeInteger;

// xsd:explicitGroup
// group type for the three kinds of model group (sequence, choice, all)
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]        from type xsd:annotated
//      Choice [0..*]                from group xsd:nestedParticle
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
//
// Used by
// Element xsd:choice
// Element xsd:sequence
//
// Type inheritance chain
// xsd:anyType
// xsd:openAttrs
// xsd:annotated
// xsd:explicitGroup
pub struct ExplicitGroup<'a> {
    annotation: Option<Annotation<'a>>,
    nested_particle: Vec<NestedParticle<'a>>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    min_occurs: NonNegativeInteger,
    max_occurs: MaxOccurs,
}