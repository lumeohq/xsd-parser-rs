use crate::xsd_model::include::Include;
use crate::xsd_model::import::Import;
use crate::xsd_model::redefine::Redefine;
use crate::xsd_model::annotation::Annotation;
use crate::xsd_model::AnyAttribute;
use crate::xsd_model::xsd::{AnyUri, Token};


// See http://www.w3.org/TR/xmlschema-1/#element-schema.
// Namespace: http://www.w3.org/2001/XMLSchema
// Schema document: xmlschema.xsd
// Type: Anonymous
// Properties: Global, Qualified
//
// Content
//  Sequence [1..1]
//      Choice [0..*]
//          xsd:include
//          xsd:import
//          xsd:redefine
//          xsd:annotation
//      Sequence [0..*]
//          Choice [1..1]   from group xsd:schemaTop
//                          from group xsd:redefinable
//              xsd:simpleType
//              xsd:complexType
//              xsd:group
//              xsd:attributeGroup

//              xsd:element
//              xsd:attribute
//              xsd:notation
//          xsd:annotation [0..*]

// Attributes
// Any attribute	    [0..*]		                    Namespace: ##other, Process Contents: lax	from type xsd:openAttrs
// targetNamespace	    [0..1]	xsd:anyURI
// version	            [0..1]	xsd:token
// finalDefault	        [0..1]	xsd:fullDerivationSet	Default value is "".
// blockDefault	        [0..1]	xsd:blockSet		    Default value is "".
// attributeFormDefault	[0..1]	xsd:formChoice		    Default value is "unqualified".
// elementFormDefault	[0..1]	xsd:formChoice		    Default value is "unqualified".
// id	                [0..1]	xsd:ID
// xml:lang	            [0..1]	Anonymous

// Identity constraints
// Type	Name	            Selector	                        Field(s)
// key	element	            xs:element	                        @name
// key	attribute	        xs:attribute	                    @name
// key	type	            xs:complexType|xs:simpleType        @name
// key	group	            xs:group	                        @name
// key	attributeGroup 	    xs:attributeGroup	                @name
// key	notation	        xs:notation	                        @name
// key	identityConstraint	.//xs:key|.//xs:unique|.//xs:keyref	@name
pub struct Schema<'a> {
    includes: Vec<Include<'a>>,
    imports: Vec<Import<'a>>,
    redefines: Vec<Redefine<'a>>,
    annotations: Vec<Annotation<'a>>,
    content: Vec<(SchemaTopGroup<'a>, Option<Annotation<'a>>)>,
    attributes: Vec<AnyAttribute<'a>>,
    target_namespace: Option<AnyUri<'a>>,
    version: Option<Token<'a>>,
    final_default: Option<FullDerivationSet>,

}


pub enum SchemaTopGroup<'a> {
    //SimpleType(SimpleType),
    //ComplexType(ComplexType),
    //Group(Group),
    //AttributeGroup(AttributeGroup),
    //Element(TopLevelElement),
    //Attribute(Attribute),
    //Notation(Notation),
}

pub enum FullDerivationSet {
    All,
    Subset(Vec<DerivationSubset>)
}

pub enum DerivationSubset {
    Extension,
    Restriction,
    List,
    Union
}