pub mod complex_types;
pub mod elements;
pub mod groups;
pub mod simple_types;

pub const XSD_NS_URI: &str = "http://www.w3.org/2001/XMLSchema";

pub type RawAttribute<'a> = roxmltree::Attribute<'a>;
pub type RawElement<'a> = roxmltree::Node<'a, 'a>;
pub type Namespace<'a> = roxmltree::Namespace<'a>;

pub enum MaxOccurs {
    Bounded(usize),
    Unbounded,
}

pub type XPath<'a> = &'a str;

#[derive(Debug, PartialEq)]
pub enum XsdType {
    All(subtype::All),
    Annotation,
    Any,
    AnyAttribute,
    AppInfo,
    Attribute(subtype::Attribute),
    AttributeGroup(subtype::AttributeGroup),
    Choice(subtype::Choice),
    ComplexContent,
    ComplexType(subtype::ComplexType),
    Documentation,
    Element(subtype::Element),
    Enumeration,
    Extension(subtype::Extension),
    Field,
    FractionDigits,
    Group(subtype::Group),
    Import,
    Include,
    Key,
    KeyRef,
    Length,
    List,
    MaxExclusive,
    MaxInclusive,
    MaxLength,
    MinExclusive,
    MinInclusive,
    MinLength,
    Notation,
    Pattern,
    Redefine,
    Restriction(subtype::Restriction),
    Schema,
    Selector,
    Sequence(subtype::Sequence),
    SimpleContent,
    SimpleType(subtype::SimpleType),
    TotalDigits,
    Union,
    Unique,
    WhiteSpace,
}

pub mod subtype {

    #[derive(Debug, PartialEq)]
    pub enum All {
        AllType,
        Anonymous,
    }

    #[derive(Debug, PartialEq)]
    pub enum Attribute {
        TopLevelAttributeType,
        LocalAttributeType,
    }

    #[derive(Debug, PartialEq)]
    pub enum AttributeGroup {
        AttributeGroupRef,
        NamedAttributeGroup,
    }

    #[derive(Debug, PartialEq)]
    pub enum Choice {
        ExplicitGroup,
        SimpleExplicitGroup,
    }

    #[derive(Debug, PartialEq)]
    pub enum ComplexType {
        TopLevelComplexType,
        LocalComplexType,
    }

    #[derive(Debug, PartialEq)]
    pub enum Element {
        TopLevelElement,
        LocalElement,
        NarrowMaxMin,
    }

    #[derive(Debug, PartialEq)]
    pub enum Extension {
        SimpleExtensionType,
        ExtensionType,
    }

    #[derive(Debug, PartialEq)]
    pub enum Group {
        NamedGroup,
        NamedGroupRef,
    }

    #[derive(Debug, PartialEq)]
    pub enum Restriction {
        SimpleRestrictionType,
        ComplexRestrictionType,
        Anonymous,
    }

    #[derive(Debug, PartialEq)]
    pub enum Sequence {
        SimpleExplicitGroup,
        ExplicitGroup,
    }

    #[derive(Debug, PartialEq)]
    pub enum SimpleType {
        LocalSimpleType,
        TopLevelSimpleType,
    }
}
