pub mod any_uri;
pub mod base64binary;
pub mod block_set;
pub mod derivation_set;
pub mod form_choice;
pub mod full_derivation_set;
pub mod id;
pub mod language;
pub mod ncname;
pub mod non_negative_integer;
pub mod positive_integer;
pub mod public;
pub mod qname;
pub mod simple_derivation_set;
pub mod token;

pub type AnySimpleType<'a> = &'a str;
pub type Id<'a> = Option<id::Id<'a>>;

