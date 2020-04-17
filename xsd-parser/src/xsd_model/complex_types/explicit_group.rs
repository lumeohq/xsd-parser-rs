use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::groups::nested_particle::NestedParticle;
use crate::xsd_model::simple_types::non_negative_integer::NonNegativeInteger;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::{MaxOccurs, RawAttribute};

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
#[derive(Debug)]
pub struct ExplicitGroup<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub nested_particle: Vec<NestedParticle<'a>>,
    pub attributes: Vec<RawAttribute<'a>>,
    pub id: Id<'a>,
    pub min_occurs: NonNegativeInteger,
    pub max_occurs: MaxOccurs,
}

impl<'a> Default for ExplicitGroup<'a> {
    fn default() -> Self {
        ExplicitGroup {
            annotation: None,
            nested_particle: vec![],
            attributes: vec![],
            id: None,
            min_occurs: NonNegativeInteger(1),
            max_occurs: MaxOccurs::Bounded(NonNegativeInteger(1)),
        }
    }
}
