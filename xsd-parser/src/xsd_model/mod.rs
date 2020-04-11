pub mod schema;
pub mod include;
pub mod annotation;
pub mod import;
pub mod redefine;
pub mod notation;
pub mod group;
pub mod attribute_group;
pub mod simple_type;
pub mod restriction;
pub mod facets;
pub mod list;
pub mod union;
pub mod attribute;
pub mod complex_type;
pub mod any_attribute;
pub mod any;
pub mod simple_content;
pub mod common_groups;
pub mod extension;
pub mod all;


pub type RawAttribute<'a> = roxmltree::Attribute<'a>;
pub type RawElement<'a> = roxmltree::Node<'a, 'a>;

pub enum FormChoice{
    Qualified,
    Unqualified
}

pub enum MaxOccurs {
    Bounded(usize),
    Unbounded,
}

impl Default for FormChoice {
    fn default() -> Self {
        FormChoice::Unqualified
    }
}

pub mod xsd {
    // Pattern: [\i-[:]][\c-[:]]* (Defined in type xsd:NCName)
    // White Space: collapse (Defined in type xsd:token)
    pub type Id<'a> = Option<&'a str>;

    // Pattern: [\i-[:]][\c-[:]]*
    // White Space: collapse
    pub type NCName<'a> = &'a str;

    // Pattern: \i\c*
    // White Space: collapse (Defined in type xsd:token)
    pub type Name<'a> = &'a str;

    //TODO: maybe replace with http::uri::Uri
    pub type AnyUri<'a> = &'a str;

    // Pattern: [a-zA-Z]{1,8}(-[a-zA-Z0-9]{1,8})*
    // White Space: collapse (Defined in type xsd:token)
    pub type Language<'a> = &'a str;

    // Based on xsd:normalizedString
    // White Space: collapse
    pub type Token<'a> = &'a str;

    // Based on xsd:token
    // Pattern: \c+
    // White Space: collapse (Defined in type xsd:token)
    pub type NmToken<'a> = &'a str;

    //A public identifier, per ISO 8879
    pub type Public<'a> = Token<'a>;

    // The type xsd:anySimpleType is the base type from which all other
    // built-in types are derived. Any value (including an empty value)
    // is allowed for xsd:anySimpleType.
    pub type AnySimpleType<'a> = &'a str;

    // The type xsd:QName represents an XML namespace-qualified name.
    // A xsd:QName value consists of a prefix and a local part,
    // separated by a colon, both of which are NCName values.
    // The prefix and colon are optional, but if they are not present,
    // it is assumed that either the name is namespace-qualified
    // by other means (e.g., by a default namespace declaration),
    // or the name is not in a namespace.
    pub struct QName<'a> {
        pub prefix: Option<&'a str>,
        pub name: &'a str
    }

    impl<'a> QName<'a> {
        pub fn new(name: &'a str) -> Self {
            match name.find(':') {
                Some(index) => Self{prefix: Some(&name[0..index]), name: &name[index + 1..]},
                None => Self{prefix: None, name},
            }
        }
    }

    // Based on xsd:nonNegativeInteger
    // Minimum Inclusive: 1
    // Fraction Digits: 0 (Defined in type xsd:integer)
    // Pattern: [\-+]?[0-9]+ (Defined in type xsd:integer)
    // White Space: collapse (Defined in type xsd:decimal)
    pub type PositiveInteger = usize;

    // Based on xsd:integer
    // Minimum Inclusive: 0
    // Fraction Digits: 0 (Defined in type xsd:integer)
    // Pattern: [\-+]?[0-9]+ (Defined in type xsd:integer)
    // White Space: collapse (Defined in type xsd:decimal)
    pub type NonNegativeInteger = usize;
}