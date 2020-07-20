use std::borrow::Cow;
use crate::abstract_code_model::comment::Comment;
use crate::abstract_code_model::{Entity, TypeName};


#[derive(Default, Debug)]
pub struct Structure<'a> {
    pub name: Option<Cow<'a, str>>,
    pub comment: Comment<'a>,
    pub fields: Vec<StructField<'a>>,
    pub subtypes: Vec<Entity<'a>>,
}

#[derive(Default, Debug)]
pub struct StructField<'a> {
    pub name: Option<Cow<'a, str>>,
    pub type_name: Option<TypeName<'a>>,
    pub comment: Comment<'a>,
    pub entities: Vec<Entity<'a>>,
    pub modifiers: Vec<TypeModifier>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeModifier {
    None,
    Array,
    Option,
    Recursive,
    Empty,
}


#[cfg(test)]
mod test {

    #[test]
    fn test() {

    }
}