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


pub type AnyAttribute<'a> = roxmltree::Attribute<'a>;
pub type AnyElement<'a> = roxmltree::Node<'a, 'a>;

pub mod xsd {
    // Pattern: [\i-[:]][\c-[:]]* (Defined in type xsd:NCName)
    //White Space: collapse (Defined in type xsd:token)
    pub type Id<'a> = &'a str;

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
}