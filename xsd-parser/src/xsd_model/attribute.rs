use crate::xsd_model::annotation::Annotation;
use crate::xsd_model::simple_type::LocalSimpleType;
use crate::xsd_model::{RawAttribute, FormChoice};
use crate::xsd_model::xsd::{Id, NCName, QName};

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
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// name	            [1..1]	xsd:NCName
// type	            [0..1]	xsd:QName
// default	        [0..1]	xsd:string
// fixed	        [0..1]	xsd:string
pub struct TopLevelAttribute<'a> {
    annotation: Option<Annotation<'a>>,
    simple_type: Option<LocalSimpleType<'a>>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    name: NCName<'a>,
    type_: Option<QName<'a>>,
    default: Option<&'a str>,
    fixed: Option<&'a str>
}



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
pub struct LocalAttribute<'a> {
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