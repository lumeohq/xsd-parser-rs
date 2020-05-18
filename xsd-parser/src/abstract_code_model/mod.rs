pub mod comment;
pub mod module;
pub mod r#struct;
pub mod r#enum;
pub mod alias;

use crate::abstract_code_model::comment::Comment;
use crate::abstract_code_model::module::Module;
use crate::abstract_code_model::r#struct::Struct;
use crate::abstract_code_model::r#enum::Enum;
use crate::abstract_code_model::alias::Alias;


#[derive(Debug)]
pub enum Entity<'a> {
    Module(Module<'a>),
    Comment(Comment<'a>),
    Struct(Struct<'a>),
    Alias(Alias<'a>),
    Enum(Enum<'a>)
}
