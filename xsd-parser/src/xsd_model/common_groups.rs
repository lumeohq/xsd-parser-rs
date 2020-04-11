use crate::xsd_model::attribute_group::AttributeGroup;
use crate::xsd_model::any_attribute::AnyAttribute;


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