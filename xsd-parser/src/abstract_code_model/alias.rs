use std::borrow::Cow;
use crate::abstract_code_model::comment::Comment;

#[derive(Debug, Default)]
pub struct Alias<'a> {
    pub name: Cow<'a, str>,
    pub type_name: Cow<'a, str>,
    pub comment: Comment<'a>,
}