use crate::xsd_model::complex_types::local_simple_type::LocalSimpleType;
use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::simple_types::ncname::NCName;
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::RawAttribute;

// xsd:topLevelAttributeType
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]    from type xsd:annotated
//      xsd:simpleType [0..1]
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// name	            [1..1]	xsd:NCName
// type	            [0..1]	xsd:QName
// default	        [0..1]	xsd:string
// fixed	        [0..1]	xsd:string
//
// Used by
// Element xsd:attribute
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:topLevelAttributeType
#[derive(Debug)]
pub struct TopLevelAttributeType<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub simple_type: Option<LocalSimpleType<'a>>,
    pub attributes: Vec<RawAttribute<'a>>,
    pub id: Id<'a>,
    pub name: NCName<'a>,
    pub type_: Option<QName<'a>>,
    pub default: Option<&'a str>,
    pub fixed: Option<&'a str>,
}
