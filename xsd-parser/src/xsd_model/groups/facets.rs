use crate::xsd_model::elements::enumeration::Enumeration;
use crate::xsd_model::elements::fraction_digits::FractionDigits;
use crate::xsd_model::elements::length::Length;
use crate::xsd_model::elements::max_exclusive::MaxExclusive;
use crate::xsd_model::elements::max_inclusive::MaxInclusive;
use crate::xsd_model::elements::max_length::MaxLength;
use crate::xsd_model::elements::min_exclusive::MinExclusive;
use crate::xsd_model::elements::min_inclusive::MinInclusive;
use crate::xsd_model::elements::min_length::MinLength;
use crate::xsd_model::elements::pattern::Pattern;
use crate::xsd_model::elements::total_digits::TotalDigits;
use crate::xsd_model::elements::white_space::WhiteSpace;

// xsd:facets
//        We should use a substitution group for facets, but
//        that's ruled out because it would allow users to
//        add their own, which we're not ready for yet.
//
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
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
//
// Used in
// Group xsd:simpleRestrictionModel
pub enum Facets<'a> {
    MinExclusive(MinExclusive<'a>),
    MinInclusive(MinInclusive<'a>),
    MaxExclusive(MaxExclusive<'a>),
    MaxInclusive(MaxInclusive<'a>),
    TotalDigits(TotalDigits<'a>),
    FractionDigits(FractionDigits<'a>),
    Length(Length<'a>),
    MinLength(MinLength<'a>),
    MaxLength(MaxLength<'a>),
    Enumeration(Enumeration<'a>),
    WhiteSpace(WhiteSpace<'a>),
    Pattern(Pattern<'a>),
}
