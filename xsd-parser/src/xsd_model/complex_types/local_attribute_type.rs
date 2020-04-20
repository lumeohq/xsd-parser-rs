use crate::xsd_model::complex_types::local_simple_type::LocalSimpleType;
use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::simple_types::form_choice::FormChoice;
use crate::xsd_model::simple_types::ncname::NCName;
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::RawAttribute;

// xsd:localAttributeType
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]    from type xsd:annotated
//      xsd:simpleType [0..1]
//
// Attributes
// Any attribute	[0..*]		Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		from type xsd:annotated
// name	            [0..1]	xsd:NCName
// ref	            [0..1]	xsd:QName
// type	            [0..1]	xsd:QName
// use	            [0..1]	Anonymous		Default value is "optional".
// default	        [0..1]	xsd:string
// fixed	        [0..1]	xsd:string
// form	            [0..1]	xsd:formChoice
//
// Used by
// Element xsd:attribute
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:localAttributeType
#[derive(Debug)]
pub struct LocalAttributeType<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub simple_type: Option<LocalSimpleType<'a>>,
    pub attributes: Vec<RawAttribute<'a>>,
    pub id: Id<'a>,
    pub name: Option<NCName<'a>>,
    pub ref_: Option<QName<'a>>,
    pub type_: Option<QName<'a>>,
    pub use_: UseType,
    pub default: Option<&'a str>,
    pub fixed: Option<&'a str>,
    pub form: Option<FormChoice>,
}

#[derive(Debug, PartialEq)]
pub enum UseType {
    Optional,
    Prohibited,
    Required,
}

impl Default for UseType {
    fn default() -> Self {
        UseType::Optional
    }
}

impl UseType {
    pub fn parse(s: &str) -> Result<Self, String> {
        Ok(match s {
            "optional" => Self::Optional,
            "prohibited" => Self::Prohibited,
            "required" => Self::Required,
            _ => return Err(format!("Error UseType parsing. Invalid value: {}", s)),
        })
    }
}
