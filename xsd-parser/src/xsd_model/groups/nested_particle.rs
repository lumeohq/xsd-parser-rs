use crate::xsd_model::complex_types::named_group_ref::NamedGroupRef;
use crate::xsd_model::elements::any::Any;
use crate::xsd_model::elements::choice::Choice;
use crate::xsd_model::elements::element::LocalElement;
use crate::xsd_model::elements::sequence::Sequence;

// xsd:nestedParticle
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Choice [1..1]
//      xsd:element
//      xsd:group
//      xsd:choice
//      xsd:sequence
//      xsd:any
//
// Used in
// Type xsd:explicitGroup (Elements xsd:choice, xsd:sequence)
// Type xsd:simpleExplicitGroup (Elements xsd:choice, xsd:sequence)
#[derive(Debug)]
pub enum NestedParticle<'a> {
    Element(Box<LocalElement<'a>>),
    Group(NamedGroupRef<'a>),
    Choice(Choice<'a>),
    Sequence(Sequence<'a>),
    Any(Any<'a>),
}
