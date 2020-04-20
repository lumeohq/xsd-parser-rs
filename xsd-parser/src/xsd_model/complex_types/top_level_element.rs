use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::groups::element_model::ElementModel;
use crate::xsd_model::simple_types::block_set::BlockSet;
use crate::xsd_model::simple_types::derivation_set::DerivationSet;
use crate::xsd_model::simple_types::ncname::NCName;
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::RawAttribute;

// xsd:topLevelElement
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
// Any attribute	    [0..*]		                Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	                [0..1]	xsd:ID		                                                    from type xsd:annotated
// name	                [1..1]	xsd:NCName
// type	                [0..1]	xsd:QName
// substitutionGroup	[0..1]	xsd:QName
// default	            [0..1]	xsd:string
// fixed	            [0..1]	xsd:string
// nillable	            [0..1]	xsd:boolean		    Default value is "false".
// abstract	            [0..1]	xsd:boolean		    Default value is "false".
// final	            [0..1]	xsd:derivationSet
// block	            [0..1]	xsd:blockSet
//
// Used by
// Element xsd:element
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:topLevelElement
#[derive(Debug, Default)]
pub struct TopLevelElement<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub model: ElementModel<'a>,
    pub attributes: Vec<RawAttribute<'a>>,
    pub id: Id<'a>,
    pub name: NCName<'a>,
    pub type_: Option<QName<'a>>,
    pub substitution_group: Option<QName<'a>>,
    pub default: Option<&'a str>,
    pub fixed: Option<&'a str>,
    pub nillable: bool,
    pub abstract_: bool,
    pub final_: Option<DerivationSet>,
    pub block: Option<BlockSet>,
}
