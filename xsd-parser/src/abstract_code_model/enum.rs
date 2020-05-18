use std::borrow::Cow;
use crate::abstract_code_model::comment::Comment;
use crate::abstract_code_model::Entity;

#[derive(Debug, Default)]
pub struct Enum<'a> {
    pub name: Option<Cow<'a, str>>,
    pub variants: Vec<Variant<'a>>,
    pub comment: Comment<'a>,
    pub type_name: Option<Cow<'a, str>>,
    pub subtypes: Vec<Entity<'a>>,
}

#[derive(Debug, Default)]
pub struct Variant<'a> {
    pub name: Option<Cow<'a, str>>,
    pub comment: Comment<'a>,
    pub value: String,
    pub type_name: Option<Cow<'a, str>>,
}