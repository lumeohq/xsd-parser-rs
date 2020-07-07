use std::borrow::Cow;
use crate::abstract_code_model::comment::Comment;

pub struct Import<'a> {
    pub name: Cow<'a, str>,
    pub location: &'a str,
    pub comment: Comment<'a>,
}