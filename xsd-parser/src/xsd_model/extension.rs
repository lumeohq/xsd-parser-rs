use crate::xsd_model::annotation::Annotation;
use crate::xsd_model::common_groups::{AttrDecls, TypeDefParticle};
use crate::xsd_model::RawAttribute;
use crate::xsd_model::xsd::{Id, QName};

// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:simpleExtensionType
// Properties: Local, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]       from type xsd:annotated
//      Choice [0..*]       from group xsd:attrDecls
//          xsd:attribute
//          xsd:attributeGroup
//      xsd:anyAttribute [0..1]
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// base	            [1..1]	xsd:QName
//
// Used in
// Anonymous type of element xsd:simpleContent
pub struct SimpleExtension<'a> {
    annotation: Option<Annotation<'a>>,
    attr_decls: AttrDecls<'a>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    base: QName<'a>
}


// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:extensionType
// Properties: Local, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]       from type xsd:annotated
//      Choice [0..1]               from group xsd:typeDefParticle
//          xsd:group
//          xsd:all    An "all" group that allows elements to appear in any order. Unlike other group types, does not allow other groups as children, only elements.
//          xsd:choice
//          xsd:sequence
//      Choice [0..*]       from group xsd:attrDecls
//          xsd:attribute
//          xsd:attributeGroup
//      xsd:anyAttribute [0..1]
//
// Attributes
// Any attribute	[0..*]		Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	[0..1]	xsd:ID		from type xsd:annotated
// base	[1..1]	xsd:QName
//
// Used in
// Anonymous type of element xsd:complexContent
pub struct Extension<'a> {
    annotation: Option<Annotation<'a>>,
    type_def_particle: TypeDefParticle<'a>,
    attr_decls: AttrDecls<'a>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    base: QName<'a>
}