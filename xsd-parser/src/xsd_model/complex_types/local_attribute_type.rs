use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::complex_types::local_simple_type::LocalSimpleType;
use crate::xsd_model::RawAttribute;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::simple_types::ncname::NCName;
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::simple_types::form_choice::FormChoice;

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
pub struct LocalAttributeType<'a> {
    annotation: Option<Annotation<'a>>,
    simple_type: Option<LocalSimpleType<'a>>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    name: NCName<'a>,
    ref_: Option<QName<'a>>,
    type_: Option<QName<'a>>,
    use_: UseType,
    default: &'a str,
    fixed: &'a str,
    form: Option<FormChoice>,
}

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