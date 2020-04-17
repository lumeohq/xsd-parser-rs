use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::groups::simple_derivation::SimpleDerivation;
use crate::xsd_model::simple_types::ncname::NCName;
use crate::xsd_model::simple_types::simple_derivation_set::SimpleDerivationSet;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::RawAttribute;

// xsd:topLevelSimpleType
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]
//      Choice [1..1]           from group xsd:simpleDerivation
//          xsd:restriction
//          xsd:list
//          xsd:union
//
// Attributes
// id	            [0..1]	xsd:ID		                                                    from type xsd:annotated
// final	        [0..1]	xsd:simpleDerivationSet		                                    from type xsd:simpleType
// name	            [1..1]	xsd:NCName	            Required at the top level
// Any attribute	[0..*]		                    Namespace: ##other, Process Contents: lax
//
// Used by
// Element xsd:simpleType
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:simpleType
//                  xsd:topLevelSimpleType
#[derive(Debug)]
pub struct TopLevelSimpleType<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub content_choice: SimpleDerivation<'a>,
    pub id: Id<'a>,
    pub final_: Option<SimpleDerivationSet>,
    pub name: NCName<'a>,
    pub attributes: Vec<RawAttribute<'a>>,
}
