use std::borrow::Cow;
use crate::abstract_code_model::comment::Comment;
use crate::abstract_code_model::TypeName;

#[derive(Debug, Default)]
pub struct Alias<'a> {
    pub name: Cow<'a, str>,
    pub type_name: TypeName<'a>,
    pub comment: Comment<'a>,
}