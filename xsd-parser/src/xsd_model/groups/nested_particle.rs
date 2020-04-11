use crate::xsd_model::elements::any::Any;

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
pub enum NestedParticle<'a> {
    //Element(LocalElement<'a>), // TODO: fix
    Group(NamedGroupRef<'a>),
    Choice(Choice<'a>),
    Sequence(Sequence<'a>),
    Any(Any<'a>),
}
