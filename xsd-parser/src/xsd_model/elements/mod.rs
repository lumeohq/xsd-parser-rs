pub mod all;
pub mod annotation;
pub mod any;
pub mod any_attribute;
pub mod app_info;
pub mod attribute;
pub mod attribute_group;
pub mod choice;
pub mod complex_content;
pub mod complex_type;
pub mod documentation;
pub mod element;
pub mod enumeration;
pub mod extension;
pub mod field;
pub mod fraction_digits;
pub mod group;
pub mod import;
pub mod include;
pub mod key;
pub mod key_ref;
pub mod length;
pub mod list;
pub mod max_exclusive;
pub mod max_inclusive;
pub mod max_length;
pub mod min_exclusive;
pub mod min_inclusive;
pub mod min_length;
pub mod notation;
pub mod pattern;
pub mod redefine;
pub mod restriction;
pub mod schema;
pub mod selector;
pub mod sequence;
pub mod simple_content;
pub mod simple_type;
pub mod total_digits;
pub mod union;
pub mod unique;
pub mod white_space;


#[derive(Debug, PartialEq)]
pub enum ElementType {
    All(All),
    Annotation,
    Any,
    AnyAttribute,
    AppInfo,
    Attribute(Attribute),
    AttributeGroup(AttributeGroup),
    Choice(Choice),
    ComplexContent,
    ComplexType(ComplexType),
    Documentation,
    Element(Element),
    Enumeration,
    Extension(Extension),
    Field,
    FractionDigits,
    Group(Group),
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
    Restriction(Restriction),
    Schema,
    Selector,
    Sequence(Sequence),
    SimpleContent,
    SimpleType(SimpleType),
    TotalDigits,
    Union,
    Unique,
    WhiteSpace,
}

pub enum All {
    AllType,
    Anonymous
}

pub enum Attribute{
    TopLevelAttributeType,
    LocalAttributeType
}

pub enum AttributeGroup {
    AttributeGroupRef,
    NamedAttributeGroup
}

pub enum Choice {
    ExplicitGroup,
    SimpleExplicitGroup
}

pub enum ComplexType {
    TopLevelComplexType,
    LocalComplexType
}

pub enum Element {
    TopLevelElement,
    LocalElement,
    NarrowMaxMin
}

pub enum Extension {
    SimpleExtensionType,
    ExtensionType,
}

pub enum Group {
    NamedGroup,
    NamedGroupRef,
}

pub enum Restriction {
    SimpleRestrictionType,
    ComplexRestrictionType,
    Anonymous
}

pub enum Sequence {
    SimpleExplicitGroup,
    ExplicitGroup,
}

pub enum SimpleType {
    LocalSimpleType,
    TopLevelSimpleType,
}


