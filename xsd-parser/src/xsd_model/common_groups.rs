use crate::xsd_model::attribute_group::AttributeGroup;
use crate::xsd_model::any_attribute::AnyAttribute;
use crate::xsd_model::simple_type::LocalSimpleType;
use crate::xsd_model::facets::FacetGroup;


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
    Attribute(Attribute<'a>),
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