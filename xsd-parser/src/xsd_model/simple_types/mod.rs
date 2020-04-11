pub mod any_uri;
pub mod base64binary;
mod id;
pub mod non_negative_integer;
pub mod ncname;
pub mod qname;
pub mod language;
pub mod form_choice;
pub mod derivation_set;
pub mod block_set;
pub mod simple_derivation_set;
pub mod full_derivation_set;
pub mod token;
pub mod positive_integer;

pub type AnySimpleType<'a> = &'a str;
pub type Id<'a> = Option<id::Id<'a>>;

