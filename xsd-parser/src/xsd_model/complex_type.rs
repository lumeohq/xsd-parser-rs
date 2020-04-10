use crate::xsd_model::annotation::Annotation;

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
pub struct TopLevelComplexType<'a>{
    annotation: Option<Annotation<'a>>,

}