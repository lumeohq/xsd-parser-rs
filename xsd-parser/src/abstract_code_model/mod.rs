pub mod comment;
pub mod module;
pub mod r#struct;
pub mod alias;

use crate::abstract_code_model::comment::Comment;
use crate::abstract_code_model::module::Module;
use crate::abstract_code_model::r#struct::Struct;


#[derive(Debug)]
pub enum Entity<'a> {
    Module(Module<'a>),
    Comment(Comment<'a>),
    Struct(Struct<'a>),

}
