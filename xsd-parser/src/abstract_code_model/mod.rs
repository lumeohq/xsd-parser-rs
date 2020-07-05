pub mod alias;
pub mod comment;
pub mod module;
pub mod r#enum;
pub mod r#struct;

use crate::abstract_code_model::alias::Alias;
use crate::abstract_code_model::comment::Comment;
use crate::abstract_code_model::module::Module;
use crate::abstract_code_model::r#enum::Enum;
use crate::abstract_code_model::r#struct::Struct;


#[derive(Debug)]
pub enum Entity<'a> {
    Alias(Alias<'a>),
    Comment(Comment<'a>),
    Enum(Enum<'a>),
    Module(Module<'a>),
    Struct(Struct<'a>),
}
