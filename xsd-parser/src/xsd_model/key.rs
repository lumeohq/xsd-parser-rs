use crate::xsd_model::annotation::Annotation;
use crate::xsd_model::RawAttribute;
use crate::xsd_model::xsd::{Id, NCName};
use crate::xsd_model::selector::Selector;
use crate::xsd_model::field::Field;

// See http://www.w3.org/TR/xmlschema-1/#element-key.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: xsd:keybase
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]           from type xsd:annotated
//      xsd:selector [1..1]
//      xsd:field [1..*]
//
// Attributes
// Any attribute	[0..*]		        Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// name	            [1..1]	xsd:NCName
//
// Used in
// Group xsd:identityConstraint
// Group xsd:elementModel via reference to xsd:identityConstraint
// Type xsd:localElement via reference to xsd:elementModel (Element xsd:element)
// Type xsd:narrowMaxMin via reference to xsd:elementModel (Element xsd:element)
// Type xsd:topLevelElement via reference to xsd:elementModel (Element xsd:element)
pub struct Key<'a> {
    annotation: Option<Annotation<'a>>,
    selector: Selector<'a>,
    fields: Vec<Field<'a>>,
    attributes: Vec<RawAttribute<'a>>,
    id: Id<'a>,
    name: NCName<'a>,
}