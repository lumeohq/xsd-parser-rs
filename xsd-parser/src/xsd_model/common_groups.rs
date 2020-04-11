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


// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      Choice [0..*]
//          xsd:attribute
//          xsd:attributeGroup
//      xsd:anyAttribute [0..1]
// Used in
// Group xsd:complexTypeModel
// Type xsd:complexRestrictionType (Element xsd:restriction)
// Type xsd:extensionType (Element xsd:extension)
// Type xsd:namedAttributeGroup (Element xsd:attributeGroup)
// Type xsd:simpleExtensionType (Element xsd:extension)
// Type xsd:simpleRestrictionType (Element xsd:restriction)
pub struct AttrDecls<'a> {
    choices: Vec<AttrChoice<'a>>,
    any_attribute: Option<AnyAttribute<'a>>
}

pub enum AttrChoice<'a> {
    Attribute(LocalAttribute<'a>),
    AttributeGroup(AttributeGroup<'a>)
}


// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
//
// Content
//  Sequence [1..1]
//      xsd:simpleType [0..1]
//      Choice [0..*]       from group xsd:facets
//          xsd:minExclusive
//          xsd:minInclusive
//          xsd:maxExclusive
//          xsd:maxInclusive
//          xsd:totalDigits
//          xsd:fractionDigits
//          xsd:length
//          xsd:minLength
//          xsd:maxLength
//          xsd:enumeration
//          xsd:whiteSpace
//          xsd:pattern
//
// Used in
// Anonymous type of element xsd:restriction
// Type xsd:simpleRestrictionType (Element xsd:restriction)
pub struct SimpleRestrictionModel<'a> {
    simple_type: Option<LocalSimpleType<'a>>,
    facets: Option<FacetGroup<'a>>
}


// We should use a substitution group for facets, but
// that's ruled out because it would allow users to
// add their own, which we're not ready for yet.
//
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
//
// Content
//  Choice [1..1]
//      xsd:minExclusive
//      xsd:minInclusive
//      xsd:maxExclusive
//      xsd:maxInclusive
//      xsd:totalDigits
//      xsd:fractionDigits
//      xsd:length
//      xsd:minLength
//      xsd:maxLength
//      xsd:enumeration
//      xsd:whiteSpace
//      xsd:pattern
pub enum FacetGroup<'a> {
    MinExclusive(Facet<'a>),
    MinInclusive(Facet<'a>),
    MaxExclusive(Facet<'a>),
    MaxInclusive(Facet<'a>),
    TotalDigits(TotalDigits<'a>),
    FractionDigits(NumFacet<'a>),
    Length(NumFacet<'a>),
    MinLength(NumFacet<'a>),
    MaxLength(NumFacet<'a>),
    Enumeration(Enumeration<'a>),
    WhiteSpace(WhiteSpace<'a>),
    Pattern(Pattern<'a>)
}


// 'complexType' uses this
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
//
// Content
//  Choice [1..1]
//      xsd:group
//      xsd:all    An "all" group that allows elements to appear in any order. Unlike other group types, does not allow other groups as children, only elements.
//      xsd:choice
//      xsd:sequence
//
// Used in
// Group xsd:complexTypeModel
// Type xsd:complexRestrictionType (Element xsd:restriction)
// Type xsd:extensionType (Element xsd:extension)
pub enum TypeDefParticle<'a> {
    Group(Group<'a>),
    All(AllType<'a>),
    Choice(Choice<'a>),
    Sequence(Sequence<'a>)
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
pub enum ComplexTypeModel<'a> {
    SimpleContent(SimpleContent<'a>),
    ComplexContent(LocalComplexType<'a>),
    TypeDefParticle(Option<TypeDefParticle<'a>>, Vec<AttrDecls<'a>>)
}