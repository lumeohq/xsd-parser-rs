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
//
// Used in
// Group xsd:nestedParticle
// Group xsd:typeDefParticle
// Group xsd:complexTypeModel via reference to xsd:typeDefParticle
// Type xsd:complexType via reference to xsd:complexTypeModel
// Type xsd:complexRestrictionType via reference to xsd:typeDefParticle (Element xsd:restriction)
// Type xsd:extensionType via reference to xsd:typeDefParticle (Element xsd:extension)
// Type xsd:localComplexType via reference to xsd:complexTypeModel (Element xsd:complexType)
// Type xsd:topLevelComplexType via reference to xsd:complexTypeModel (Element xsd:complexType)
// Type xsd:explicitGroup via reference to xsd:nestedParticle (Elements xsd:choice, xsd:sequence)
// Type xsd:simpleExplicitGroup via reference to xsd:nestedParticle (Elements xsd:choice, xsd:sequence)
pub struct Sequence<'a> {
    annotation: Option<Annotation<'a>>,
    nested_particle: Vec<NestedParticle<'a>>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    min_occurs: NonNegativeInteger,
    max_occurs: MaxOccurs,
}


// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:simpleExplicitGroup
// Properties: Local, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]           from type xsd:annotated
//      Choice [0..*]           from group xsd:nestedParticle
//          xsd:element
//          xsd:group
//          xsd:choice
//          xsd:sequence
//          xsd:any
//
// Attributes
// Any attribute	[0..*]		    Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                        from type xsd:annotated
//
// Used in
// Type xsd:namedGroup (Element xsd:group)
pub struct SimpleSequence<'a> {
    annotation: Option<Annotation<'a>>,
    nested_particle: Vec<NestedParticle<'a>>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
}
