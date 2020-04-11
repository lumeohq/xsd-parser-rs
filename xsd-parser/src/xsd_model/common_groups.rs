use crate::xsd_model::attribute_group::AttributeGroup;
use crate::xsd_model::any_attribute::AnyAttribute;
use crate::xsd_model::simple_type::LocalSimpleType;
use crate::xsd_model::facets::{Facet, TotalDigits, NumFacet, Enumeration, WhiteSpace, Pattern};
use crate::xsd_model::attribute::LocalAttribute;
use crate::xsd_model::group::Group;
use crate::xsd_model::all::AllType;
use crate::xsd_model::choice::Choice;
use crate::xsd_model::any::Any;
use crate::xsd_model::sequence::Sequence;
use crate::xsd_model::simple_content::SimpleContent;
use crate::xsd_model::complex_type::LocalComplexType;
use crate::xsd_model::unique::Unique;
use crate::xsd_model::key::Key;
use crate::xsd_model::key_ref::KeyRef;



pub enum AttrChoice<'a> {
    Attribute(LocalAttribute<'a>),
    AttributeGroup(AttributeGroup<'a>)
}



// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
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
    //Element(LocalElement<'a>),
    Group(Group<'a>),
    Choice(Choice<'a>),
    Sequence(Sequence<'a>),
    Any(Any<'a>),
}


// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
//
// Content
//  Choice [1..1]
//      xsd:simpleContent
//      xsd:complexContent
//      Sequence [1..1]
//          Choice [0..1]       from group xsd:typeDefParticle
//              xsd:group
//              xsd:all         An "all" group that allows elements to appear in any order. Unlike other group types, does not allow other groups as children, only elements.
//              xsd:choice
//              xsd:sequence
//          Choice [0..*]       from group xsd:attrDecls
//              xsd:attribute
//              xsd:attributeGroup
//          xsd:anyAttribute [0..1]
//
// Used in
// Type xsd:complexType
// Type xsd:localComplexType (Element xsd:complexType)
// Type xsd:topLevelComplexType (Element xsd:complexType)




// The three kinds of identity constraints, all with type of or derived from 'keybase'.
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
//
// Content
//  Choice [1..1]
//      xsd:unique
//      xsd:key
//      xsd:keyref
//
// Used in
// Group xsd:elementModel
pub enum IdentityConstraint<'a> {
    Unique(Unique<'a>),
    Key(Key<'a>),
    KeyRef(KeyRef<'a>),
}