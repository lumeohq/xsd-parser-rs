use crate::xsd_model::annotation::Annotation;
use crate::xsd_model::xsd::{Id, NCName};
use crate::xsd_model::RawAttribute;
use crate::xsd_model::restriction::SimpleTypeRestriction;
use crate::xsd_model::list::List;
use crate::xsd_model::union::Union;


// See http://www.w3.org/TR/xmlschema-2/#element-simpleType.
// Element information
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: xsd:topLevelSimpleType
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]
//      Choice [1..1]    from group xsd:simpleDerivation
//          xsd:restriction
//          xsd:list
//          xsd:union
//
// Attributes
// id	            [0..1]	xsd:ID		                                            from type xsd:annotated
// final	        [0..1]	xsd:simpleDerivationSet		                            from type xsd:simpleType
// name	            [1..1]	xsd:NCName	            Required at the top level
// Any attribute	[0..*]		                    Namespace: ##other, Process Contents: lax
pub struct TopLevelSimpleType<'a> {
    annotation: Option<Annotation<'a>>,
    content_choice: SimpleDerivation<'a>,
    id: Option<Id<'a>>,
    final_: Option<SimpleDerivationSet>,
    name: NCName<'a>,
    attributes: Vec<RawAttribute<'a>>
}


// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
// Type: xsd:localSimpleType
// Properties: Local, Qualified
//
// Content
//  Sequence [1..1]
//      xsd:annotation [0..1]
//      Choice [1..1]    from group xsd:simpleDerivation
//          xsd:restriction
//          xsd:list
//          xsd:union
//
// Attributes
// id	            [0..1]	xsd:ID		                                        from type xsd:annotated
// Any attribute	[0..*]		    Namespace: ##other, Process Contents: lax
pub struct LocalSimpleType<'a> {
    annotation: Option<Annotation<'a>>,
    content_choice: SimpleDerivation<'a>,
    id: Option<Id<'a>>,
    attributes: Vec<RawAttribute<'a>>
}


// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: datatypes.xsd
pub enum SimpleDerivation<'a> {
    Restriction(SimpleTypeRestriction<'a>),
    List(List<'a>),
    Union(Union<'a>),
}

// #all or (possibly empty) subset of {restriction, union, list}
// A utility type, not for public use
pub enum SimpleDerivationSet {
    All,
    Subset(Vec<SimpleDerivationSubset>)
}

pub enum SimpleDerivationSubset {
    Restriction,
    List,
    Union,
}