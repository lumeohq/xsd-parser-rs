use crate::xsd_model::elements::simple_type::TopLevelSimpleType;
use crate::xsd_model::elements::complex_type::TopLevelComplexType;
use crate::xsd_model::elements::group::Group;
use crate::xsd_model::elements::attribute_group::AttributeGroup;

// xsd:redefinable
// This group is for the elements which can self-redefine.
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Choice [1..1]
//      xsd:simpleType
//      xsd:complexType
//      xsd:group
//      xsd:attributeGroup
//
// Used in
// Anonymous type of element xsd:redefine
// Group xsd:schemaTop
pub enum Redefinable<'a> {
    SimpleType(TopLevelSimpleType<'a>),
    ComplexType(TopLevelComplexType<'a>),
    Group(Group<'a>),
    AttributeGroup(AttributeGroup<'a>),
}