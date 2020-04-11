pub mod any_uri;
pub mod base64binary;
mod id;
pub mod non_negative_integer;
pub mod ncname;
pub mod qname;
pub mod language;
pub mod form_choice;


pub type AnySimpleType<'a> = &'a str;
pub type Id<'a> = Option<id::Id<'a>>;

