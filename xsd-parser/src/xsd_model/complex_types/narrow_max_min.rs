use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::groups::element_model::ElementModel;
use crate::xsd_model::groups::identity_constraint::IdentityConstraint;
use crate::xsd_model::simple_types::block_set::BlockSet;
use crate::xsd_model::simple_types::form_choice::FormChoice;
use crate::xsd_model::simple_types::ncname::NCName;
use crate::xsd_model::simple_types::non_negative_integer::NonNegativeInteger;
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::{MaxOccurs, RawAttribute};

// xsd:narrowMaxMin
// restricted max/min
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]
//      Choice [0..1]    from group xsd:elementModel
//          xsd:simpleType
//          xsd:complexType
//      Choice [0..*]    from group xsd:identityConstraint
//          xsd:unique
//          xsd:key
//          xsd:keyref
//
// Attributes
// id	            [0..1]	xsd:ID		                                                from type xsd:annotated
// name	            [0..1]	xsd:NCName		                                            from type xsd:localElement
// ref	            [0..1]	xsd:QName		                                            from type xsd:localElement
// type	            [0..1]	xsd:QName		                                            from type xsd:localElement
// default	        [0..1]	xsd:string		                                            from type xsd:localElement
// fixed	        [0..1]	xsd:string		                                            from type xsd:localElement
// nillable	        [0..1]	xsd:boolean		Default value is "false".                   from type xsd:localElement
// block	        [0..1]	xsd:blockSet		                                        from type xsd:localElement
// form	            [0..1]	xsd:formChoice		                                        from type xsd:localElement
// minOccurs	    [0..1]	Anonymous		Default value is "1".
// maxOccurs	    [0..1]	Anonymous		Default value is "1".
// Any attribute	[0..*]		            Namespace: ##other, Process Contents: lax
//
// Used by
// Element xsd:element
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:localElement
//                  xsd:narrowMaxMin
#[derive(Debug)]
pub struct NarrowMaxMin<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub model: ElementModel<'a>,
    pub id: Id<'a>,
    pub name: Option<NCName<'a>>,
    pub ref_: Option<QName<'a>>,
    pub type_: Option<QName<'a>>,
    pub default: Option<&'a str>,
    pub fixed: Option<&'a str>,
    pub nillable: bool,
    pub block: Option<BlockSet>,
    pub form: Option<FormChoice>,
    pub min_occurs: NonNegativeInteger, //Anonymous in doc, probably mistake
    pub max_occurs: MaxOccurs,
    pub attributes: Vec<RawAttribute<'a>>,
}
