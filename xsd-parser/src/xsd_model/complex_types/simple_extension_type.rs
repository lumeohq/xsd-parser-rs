use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::groups::attr_decls::AttrDecls;
use crate::xsd_model::RawAttribute;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::simple_types::qname::QName;

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
pub struct SimpleExtensionType<'a> {
    annotation: Option<Annotation<'a>>,
    attr_decls: AttrDecls<'a>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    base: QName<'a>
}