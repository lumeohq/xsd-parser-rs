use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::groups::attr_decls::AttrDecls;
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::RawAttribute;

// xsd:simpleExtensionType
// attrs only
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
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
// Used by
// Element xsd:extension
//
// Type inheritance chain
// xsd:anyType
// xsd:openAttrs
// xsd:annotated
// xsd:simpleExtensionType
#[derive(Debug, Default)]
pub struct SimpleExtensionType<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub attr_decls: AttrDecls<'a>,
    pub attributes: Vec<RawAttribute<'a>>,
    pub id: Id<'a>,
    pub base: QName<'a>,
}
