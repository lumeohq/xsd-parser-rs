use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::groups::complex_type_model::ComplexTypeModel;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::RawAttribute;

// xsd:localComplexType
// Complex type information
// Namespace: http://www.w3.org/2001/XMLSchema
//
// Schema document: xmlschema.xsd
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]           from type xsd:annotated
//      Choice [1..1]                   from group xsd:complexTypeModel
//          xsd:simpleContent
//          xsd:complexContent
//              Sequence [1..1]
//                  Choice [0..1]           from group xsd:typeDefParticle
//                      xsd:group
//                      xsd:all    An "all" group that allows elements to appear in any order. Unlike other group types, does not allow other groups as children, only elements.
//                      xsd:choice
//                      xsd:sequence
//              Choice [0..*]               from group xsd:attrDecls
//                  xsd:attribute
//                  xsd:attributeGroup
//              xsd:anyAttribute [0..1]
// Attributes
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// mixed	        [0..1]	xsd:boolean	Indicates that mixed content is allowed.	Default value is "false".
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax
//
// Used by
// Element xsd:complexType
//
// Type inheritance chain
//  xsd:anyType
//      xsd:openAttrs
//          xsd:annotated
//              xsd:localComplexType
#[derive(Debug)]
pub struct LocalComplexType<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub model: Box<ComplexTypeModel<'a>>,
    pub id: Id<'a>,
    pub mixed: bool,
    pub attributes: Vec<RawAttribute<'a>>,
}
