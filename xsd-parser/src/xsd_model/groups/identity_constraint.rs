use crate::xsd_model::elements::unique::Unique;
use crate::xsd_model::elements::key::Key;
use crate::xsd_model::elements::key_ref::KeyRef;

// xsd:identityConstraint
// The three kinds of identity constraints, all with type of or derived from 'keybase'.
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
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