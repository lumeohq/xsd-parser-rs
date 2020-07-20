use std::borrow::Cow;
use crate::abstract_code_model::comment::Comment;
use crate::abstract_code_model::{Entity, TypeName};
use crate::abstract_code_model::abstract_field::AbstractField;

#[derive(Debug, Default)]
pub struct Enumeration<'a> {
    pub name: Option<Cow<'a, str>>,
    pub items: Vec<EnumItem<'a>>,
    pub comment: Comment<'a>,
    pub subtypes: Vec<Entity<'a>>,
}


pub type EnumItem<'a> = AbstractField<'a>;