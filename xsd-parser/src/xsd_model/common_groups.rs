use crate::xsd_model::attribute_group::AttributeGroup;
use crate::xsd_model::any_attribute::AnyAttribute;
use crate::xsd_model::simple_type::LocalSimpleType;
use crate::xsd_model::facets::{Facet, TotalDigits, NumFacet, Enumeration, WhiteSpace, Pattern};
use crate::xsd_model::attribute::LocalAttribute;
use crate::xsd_model::group::Group;


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
    All(All<'a>),
    Choice(Choice<'a>),
    Sequence(Sequence<'a>)
}