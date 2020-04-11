pub mod any_uri;
pub mod base64binary;
pub mod id;
pub mod non_negative_integer;
pub mod ncname;

pub type AnySimpleType<'a> = &'a str;
pub type Id<'a> = Option<id::Id<'a>>;

