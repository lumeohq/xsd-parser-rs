pub mod alias;
pub mod comment;
pub mod module;
pub mod enumeration;
pub mod structure;
pub mod abstract_field;
pub mod import;

use crate::abstract_code_model::alias::Alias;
use crate::abstract_code_model::comment::Comment;
use crate::abstract_code_model::module::Module;
use crate::abstract_code_model::enumeration::{Enumeration, EnumItem};
use crate::abstract_code_model::structure::{Structure, StructField};
use std::borrow::Cow;
use crate::abstract_code_model::abstract_field::AbstractField;


#[derive(Debug)]
pub enum Entity<'a> {
    Alias(Alias<'a>),
    Comment(Comment<'a>),
    Enum(Enumeration<'a>),
    Module(Module<'a>),
    Struct(Structure<'a>),
    StructFields(Vec<StructField<'a>>), //struct field and tuple struct field
    EnumItems(Vec<EnumItem<'a>>)
}

#[derive(Default, Debug)]
pub struct TypeName<'a> {
    pub namespace: Option<Cow<'a, str>>,
    pub name: Cow<'a, str>,
}