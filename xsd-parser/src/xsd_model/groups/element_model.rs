use crate::xsd_model::elements::complex_type::LocalComplexType;
use crate::xsd_model::elements::simple_type::LocalSimpleType;
use crate::xsd_model::groups::identity_constraint::IdentityConstraint;

// xsd:elementModel
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      Choice [0..1]
//          xsd:simpleType
//          xsd:complexType
//      Choice [0..*]        from group xsd:identityConstraint
//          xsd:unique
//          xsd:key
//          xsd:keyref
//
// Used in
// Type xsd:localElement (Element xsd:element)
// Type xsd:narrowMaxMin (Element xsd:element)
// Type xsd:topLevelElement (Element xsd:element)
#[derive(Debug)]
pub struct ElementModel<'a> {
    choice: ElementModelChoice<'a>,
    identity_constraint: IdentityConstraint<'a>,
}

#[derive(Debug)]
pub enum ElementModelChoice<'a> {
    SimpleType(LocalSimpleType<'a>),
    ComplexType(LocalComplexType<'a>),
}
