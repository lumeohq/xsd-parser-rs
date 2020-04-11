use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::{RawAttribute, MaxOccurs};
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::simple_types::non_negative_integer::NonNegativeInteger;
use crate::xsd_model::simple_types::form_choice::FormChoice;
use crate::xsd_model::groups::element_model::ElementModel;
use crate::xsd_model::simple_types::block_set::BlockSet;
use crate::xsd_model::simple_types::Id;

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
pub struct LocalElement<'a> {
    annotation: Option<Annotation<'a>>,
    model: ElementModel<'a>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    name: Option<QName<'a>>,
    ref_: Option<QName<'a>>,
    type_: Option<QName<'a>>,
    min_occurs: NonNegativeInteger,
    max_occurs: MaxOccurs,
    default: Option<&'a str>,
    fixed: Option<&'a str>,
    nillable: bool,
    block: Option<BlockSet>,
    form: Option<FormChoice>,
}