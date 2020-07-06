pub mod alias;
pub mod comment;
pub mod module;
pub mod enumeration;
pub mod r#struct;
pub mod abstract_field;

use crate::abstract_code_model::alias::Alias;
use crate::abstract_code_model::comment::Comment;
use crate::abstract_code_model::module::Module;
use crate::abstract_code_model::enumeration::Enumeration;
use crate::abstract_code_model::r#struct::Struct;
use std::borrow::Cow;


#[derive(Debug)]
pub enum Entity<'a> {
    Alias(Alias<'a>),
    Comment(Comment<'a>),
    Enum(Enumeration<'a>),
    Module(Module<'a>),
    Struct(Struct<'a>),
}

#[derive(Default, Debug)]
pub struct TypeName<'a> {
    pub namespace: Option<Cow<'a, str>>,
    pub name: Cow<'a, str>,
}