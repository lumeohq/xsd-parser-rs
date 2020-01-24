#[derive(Debug, PartialEq)]
pub enum ElementType {
    All,
    Annotation,
    Any,
    AnyAttribute,
    AppInfo,
    Attribute,
    AttributeGroup,
    Choice,
    ComplexContent,
    ComplexType,
    Documentation,
    Element,
    Extension(ExtensionType),
    Field,
    Group,
    Import,
    Include,
    Key,
    KeyRef,
    List,
    Notation,
    Redefine,
    Restriction(RestrictionType),
    Schema,
    Selector,
    Sequence,
    SimpleContent,
    SimpleType,
    Union,
    Unique,
    Facet(FacetType),
    UnknownElement(String),
    XsdError(String),
}

#[derive(Debug, PartialEq)]
pub enum FacetType {
    Enumeration,
    FractionDigits,
    Length,
    MaxExclusive,
    MaxInclusive,
    MaxLength,
    MinExclusive,
    MinInclusive,
    MinLength,
    Pattern,
    TotalDigits,
    WhiteSpace,
}

#[derive(Debug, PartialEq)]
pub enum ExtensionType {
    ComplexContent,
    SimpleContent,
}

#[derive(Debug, PartialEq)]
pub enum RestrictionType {
    ComplexContent,
    SimpleContent,
    SimpleType,
}

pub trait XmlNode {
    fn xsd_type(&self) -> ElementType;
}

impl XmlNode for roxmltree::Node<'_, '_> {
    fn xsd_type(&self) -> ElementType {
        use ElementType::*;
        match self.tag_name().name() {
            "all" => All,
            "annotation" => Annotation,
            "any" => Any,
            "anyAttribute" => AnyAttribute,
            "appInfo" => AppInfo,
            "attribute" => Attribute,
            "attributeGroup" => AttributeGroup,
            "choice" => Choice,
            "complexContent" => ComplexContent,
            "complexType" => ComplexType,
            "documentation" => Documentation,
            "element" => Element,
            "extension" => match self.parent() {
                Some(parent) => match parent.tag_name().name() {
                    "complexContent" => Extension(ExtensionType::ComplexContent),
                    "simpleContent" => Extension(ExtensionType::SimpleContent),
                    _ => XsdError(format!(
                        "Unsupported parent type ({}) for 'extension' element",
                        parent.tag_name().name()
                    )),
                },
                None => XsdError("'extension' element must have a parent".to_string()),
            },
            "field" => Field,
            "group" => Group,
            "import" => Import,
            "include" => Include,
            "key" => Key,
            "keyRef" => KeyRef,
            "list" => List,
            "notation" => Notation,
            "redefine" => Redefine,
            "restriction" => match self.parent() {
                Some(parent) => match parent.tag_name().name() {
                    "complexContent" => Restriction(RestrictionType::ComplexContent),
                    "simpleContent" => Restriction(RestrictionType::SimpleContent),
                    "simpleType" => Restriction(RestrictionType::SimpleType),
                    _ => XsdError(format!(
                        "Unsupported parent type ({}) for 'restriction' element",
                        parent.tag_name().name()
                    )),
                },
                None => XsdError("'restriction' element must have a parent".to_string()),
            },
            "schema" => Schema,
            "selector" => Selector,
            "sequence" => Sequence,
            "simpleContent" => SimpleContent,
            "simpleType" => SimpleType,
            "union" => Union,
            "unique" => Unique,

            "enumeration" => Facet(FacetType::Enumeration),
            "fractionDigits" => Facet(FacetType::FractionDigits),
            "length" => Facet(FacetType::Length),
            "maxExclusive" => Facet(FacetType::MaxExclusive),
            "maxInclusive" => Facet(FacetType::MaxInclusive),
            "maxLength" => Facet(FacetType::MaxLength),
            "minExclusive" => Facet(FacetType::MinExclusive),
            "minInclusive" => Facet(FacetType::MinInclusive),
            "minLength" => Facet(FacetType::MinLength),
            "pattern" => Facet(FacetType::Pattern),
            "totalDigits" => Facet(FacetType::TotalDigits),
            "whiteSpace" => Facet(FacetType::WhiteSpace),

            _ => UnknownElement(self.tag_name().name().to_string()),
        }
    }
}

pub trait Name {
    fn name(&self) -> Option<&str>;
}

impl Name for Option<roxmltree::Namespace<'_>> {
    fn name(&self) -> Option<& str> {
        match self {
            Some(n) => n.name(),
            None => None
        }
    }
}