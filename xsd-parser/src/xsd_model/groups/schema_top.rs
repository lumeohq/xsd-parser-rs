use crate::xsd_model::elements::simple_type::TopLevelSimpleType;
use crate::xsd_model::elements::complex_type::TopLevelComplexType;
use crate::xsd_model::elements::group::Group;
use crate::xsd_model::elements::attribute_group::AttributeGroup;
use crate::xsd_model::elements::element::TopLevelElement;
use crate::xsd_model::elements::attribute::TopLevelAttribute;
use crate::xsd_model::elements::notation::Notation;

// xsd:schemaTop
// This group is for the elements which occur freely at the top level of schemas. All of their types are based on the "annotated" type by extension.
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Choice [1..1]
//      from group xsd:redefinable
//          xsd:simpleType
//          xsd:complexType
//          xsd:group
//          xsd:attributeGroup
//      xsd:element
//      xsd:attribute
//      xsd:notation
//
// Used in
// Anonymous type of element xsd:schema
pub enum SchemaTop<'a> {
    SimpleType(Box<TopLevelSimpleType<'a>>),
    ComplexType(Box<TopLevelComplexType<'a>>),
    Group(Box<Group<'a>>),
    AttributeGroup(Box<AttributeGroup<'a>>),
    Element(Box<TopLevelElement<'a>>),
    Attribute(Box<TopLevelAttribute<'a>>),
    Notation(Notation<'a>),
}