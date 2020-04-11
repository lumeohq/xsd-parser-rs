use crate::xsd_model::annotation::Annotation;
use crate::xsd_model::common_groups::ComplexTypeModel;
use crate::xsd_model::RawAttribute;
use crate::xsd_model::xsd::{Id, QName};
use crate::xsd_model::simple_type::SimpleDerivationSet;

// See http://www.w3.org/TR/xmlschema-1/#element-complexType.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:topLevelComplexType
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]    from type xsd:annotated
//      Choice [1..1]    from group xsd:complexTypeModel
//          xsd:simpleContent
//          xsd:complexContent
//          Sequence [1..1]
//              Choice [0..1]    from group xsd:typeDefParticle
//                  xsd:group
//                  xsd:all    An "all" group that allows elements to appear in any order. Unlike other group types, does not allow other groups as children, only elements.
//                  xsd:choice
//                  xsd:sequence
//              Choice [0..*]    from group xsd:attrDecls
//                  xsd:attribute
//                  xsd:attributeGroup
//              xsd:anyAttribute [0..1]
//
// Attributes
// Any attribute	[0..*]		                Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                                    from type xsd:annotated
// name	            [1..1]	xsd:NCName
// abstract	        [0..1]	xsd:boolean		                                                Default value is "false".
// final	        [0..1]	xsd:derivationSet
// block	        [0..1]	xsd:derivationSet
// mixed	        [0..1]	xsd:boolean	        Indicates that mixed content is allowed.	Default value is "false".
//
// Used in
// Group xsd:redefinable
// Anonymous type of element xsd:redefine via reference to xsd:redefinable
// Anonymous type of element xsd:schema via reference to xsd:schemaTop
// Group xsd:schemaTop via reference to xsd:redefinable
pub struct TopLevelComplexType<'a>{
    annotation: Option<Annotation<'a>>,
    model: ComplexTypeModel<'a>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    name: QName<'a>,
    abstract_: bool,
    final_: Option<SimpleDerivationSet>,
    block: Option<SimpleDerivationSet>,
    mixed: bool,
}


// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:localComplexType
// Properties: Local, Qualified
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
// Used in
// Group xsd:elementModel
// Type xsd:localElement via reference to xsd:elementModel (Element xsd:element)
// Type xsd:narrowMaxMin via reference to xsd:elementModel (Element xsd:element)
// Type xsd:topLevelElement via reference to xsd:elementModel (Element xsd:element)
pub struct LocalComplexType<'a> {
    annotation: Option<Annotation<'a>>,
    model: Box<ComplexTypeModel<'a>>,
    id: Id<'a>,
    mixed: bool,
    attributes: Vec<RawAttribute<'a>>,
}