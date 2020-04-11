use crate::xsd_model::elements::annotation::Annotation;
use crate::xsd_model::simple_types::Id;
use crate::xsd_model::simple_types::qname::QName;
use crate::xsd_model::complex_types::simple_restriction_type::SimpleRestrictionType;
use crate::xsd_model::complex_types::complex_restriction_type::ComplexRestrictionType;
use crate::xsd_model::groups::simple_restriction_model::SimpleRestrictionModel;
use crate::xsd_model::RawAttribute;

// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]        from type xsd:annotated
//      xsd:simpleType [0..1]        from group xsd:simpleRestrictionModel
//      Choice [0..*]                from group xsd:facets
//          xsd:minExclusive
//          xsd:minInclusive
//          xsd:maxExclusive
//          xsd:maxInclusive
//          xsd:totalDigits
//          xsd:fractionDigits
//          xsd:length
//          xsd:minLength
//          xsd:maxLength
//          xsd:enumeration
//          xsd:whiteSpace
//          xsd:pattern
//
// Attributes
// Any attribute	[0..*]		    Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                        from type xsd:annotated
// base	            [0..1]	xsd:QName
//
// Used in
// Group xsd:simpleDerivation
// Type xsd:simpleType via reference to xsd:simpleDerivation
// Type xsd:localSimpleType via reference to xsd:simpleDerivation (Element xsd:simpleType)
// Type xsd:topLevelSimpleType via reference to xsd:simpleDerivation (Element xsd:simpleType)
pub struct Restriction<'a> {
    annotation: Option<Annotation<'a>>,
    model: SimpleRestrictionModel<'a>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    base: Option<QName<'a>>
}


// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:simpleRestrictionType
// Properties: Local, Qualified
//
// Used in
// Anonymous type of element xsd:simpleContent
pub type SimpleRestriction<'a>  = SimpleRestrictionType<'a>;


// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:complexRestrictionType
// Properties: Local, Qualified
//
// Used in
// Anonymous type of element xsd:complexContent

pub type ComplexRestriction<'a>  = ComplexRestrictionType<'a>;