use crate::xsd_model::complex_types::attribute_group_ref;
use crate::xsd_model::complex_types::named_attribute_group;

// xsd:attributeGroup
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:attributeGroupRef
// Properties: Local, Qualified
//
// Used in
// Group xsd:attrDecls
// Group xsd:complexTypeModel via reference to xsd:attrDecls
// Type xsd:complexType via reference to xsd:complexTypeModel
// Type xsd:complexRestrictionType via reference to xsd:attrDecls (Element xsd:restriction)
// Type xsd:extensionType via reference to xsd:attrDecls (Element xsd:extension)
// Type xsd:localComplexType via reference to xsd:complexTypeModel (Element xsd:complexType)
// Type xsd:namedAttributeGroup via reference to xsd:attrDecls (Element xsd:attributeGroup)
// Type xsd:simpleExtensionType via reference to xsd:attrDecls (Element xsd:extension)
// Type xsd:simpleRestrictionType via reference to xsd:attrDecls (Element xsd:restriction)
// Type xsd:topLevelComplexType via reference to xsd:complexTypeModel (Element xsd:complexType)
pub type AttributeGroupRef<'a> = attribute_group_ref::AttributeGroupRef<'a>;

// xsd:attributeGroup
// See http://www.w3.org/TR/xmlschema-1/#element-attributeGroup.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:namedAttributeGroup
// Properties: Global, Qualified
//
// Used in
// Group xsd:redefinable
// Anonymous type of element xsd:redefine via reference to xsd:redefinable
// Anonymous type of element xsd:schema via reference to xsd:schemaTop
// Group xsd:schemaTop via reference to xsd:redefinable
pub type AttributeGroup<'a> = named_attribute_group::NamedAttributeGroup<'a>;
