use crate::xsd_model::annotation::Annotation;
use crate::xsd_model::common_groups::ElementModel;
use crate::xsd_model::RawAttribute;
use crate::xsd_model::xsd::{Id, QName};
use crate::xsd_model::simple_type::SimpleDerivationSet;
use crate::xsd_model::schema::BlockSet;

// See http://www.w3.org/TR/xmlschema-1/#element-element.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:topLevelElement
// Properties: Global, Qualified
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
// Used in
// Group xsd:schemaTop
// Anonymous type of element xsd:schema via reference to xsd:schemaTop
pub struct TopLevelElement<'a> {
    annotation: Option<Annotation<'a>>,
    model: ElementModel<'a>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    name: QName<'a>,
    type_: Option<QName<'a>>,
    substitution_group: Option<QName<'a>>,
    default: Option<&'a str>,
    fixed: Option<&'a str>,
    nillable: bool,
    abstract_: bool,
    final_: Option<SimpleDerivationSet>,
    block: Option<BlockSet>,
}