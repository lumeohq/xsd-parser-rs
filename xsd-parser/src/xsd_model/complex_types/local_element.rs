use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::groups::element_model::ElementModel;
use crate::xsd_model::simple_types::block_set::BlockSet;
use crate::xsd_model::simple_types::form_choice::FormChoice;
use crate::xsd_model::simple_types::ncname::NCName;
use crate::xsd_model::simple_types::non_negative_integer::NonNegativeInteger;
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::{MaxOccurs, RawAttribute};
use num_bigint::ToBigUint;

// xsd:localElement
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]               from type xsd:annotated
//      Choice [0..1]                       from group xsd:elementModel
//          xsd:simpleType
//          xsd:complexType
//      Choice [0..*]                       from group xsd:identityConstraint
//          xsd:unique
//          xsd:key
//          xsd:keyref
//
// Attributes
// Any attribute	[0..*]		                    Namespace: ##other, Process Contents: lax	        from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                                                from type xsd:annotated
// name	            [0..1]	xsd:NCName
// ref	            [0..1]	xsd:QName
// type	            [0..1]	xsd:QName
// minOccurs	    [0..1]	xsd:nonNegativeInteger	minimum number of occurrences	Default value is "1". from group xsd:occurs
// maxOccurs	    [0..1]	Anonymous	            maximum number of occurrences	Default value is "1". from group xsd:occurs
// default	        [0..1]	xsd:string
// fixed	        [0..1]	xsd:string
// nillable	        [0..1]	xsd:boolean		                                        Default value is "false".
// block	        [0..1]	xsd:blockSet
// form	            [0..1]	xsd:formChoice
//
// Used by
// Element xsd:element
// Element xsd:element via derived type xsd:narrowMaxMin
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:localElement
//                  restricted by xsd:narrowMaxMin
#[derive(Debug)]
pub struct LocalElement<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub model: ElementModel<'a>,
    pub attributes: Vec<RawAttribute<'a>>,
    pub id: Id<'a>,
    pub name: Option<NCName<'a>>,
    pub ref_: Option<QName<'a>>,
    pub type_: Option<QName<'a>>,
    pub min_occurs: NonNegativeInteger,
    pub max_occurs: MaxOccurs,
    pub default: Option<&'a str>,
    pub fixed: Option<&'a str>,
    pub nillable: bool,
    pub block: Option<BlockSet>,
    pub form: Option<FormChoice>,
}

impl Default for LocalElement<'_> {
    fn default() -> Self {
        Self {
            annotation: None,
            model: Default::default(),
            attributes: vec![],
            id: None,
            name: None,
            ref_: None,
            type_: None,
            min_occurs: NonNegativeInteger::from_biguint(1.to_biguint().unwrap()),
            max_occurs: MaxOccurs::Bounded(NonNegativeInteger::from_biguint(1.to_biguint().unwrap())),
            default: None,
            fixed: None,
            nillable: false,
            block: None,
            form: None,
        }
    }
}
