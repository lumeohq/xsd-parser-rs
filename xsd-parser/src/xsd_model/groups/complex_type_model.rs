use crate::xsd_model::elements::complex_content::ComplexContent;
use crate::xsd_model::elements::simple_content::SimpleContent;
use crate::xsd_model::groups::attr_decls::AttrDecls;
use crate::xsd_model::groups::type_def_particle::TypeDefParticle;

// xsd:complexTypeModel
// Group information
// Namespace: http://www.w3.org/2001/XMLSchema
//
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
    ComplexContent(ComplexContent<'a>),
    TypeDefParticle(Option<TypeDefParticle<'a>>, Vec<AttrDecls<'a>>),
}
