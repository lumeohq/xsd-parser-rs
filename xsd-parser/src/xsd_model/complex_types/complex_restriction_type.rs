use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::groups::attr_decls::AttrDecls;
use crate::xsd_model::groups::type_def_particle::TypeDefParticle;

// xsd:complexRestrictionType
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]       from type xsd:annotated
//      Choice [0..1]               from group xsd:typeDefParticle
//          xsd:group
//          xsd:all    An "all" group that allows elements to appear in any order. Unlike other group types, does not allow other groups as children, only elements.
//          xsd:choice
//          xsd:sequence
//      Choice [0..*]           from group xsd:attrDecls
//          xsd:attribute
//          xsd:attributeGroup
//      xsd:anyAttribute [0..1]
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// base	            [1..1]	xsd:QName
//
// Used by
// Element xsd:restriction
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:complexRestrictionType
#[derive(Debug)]
pub struct ComplexRestrictionType<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub type_def_particle: TypeDefParticle<'a>,
    pub attr_decls: AttrDecls<'a>,
}
