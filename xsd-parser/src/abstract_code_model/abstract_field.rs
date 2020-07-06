use std::borrow::Cow;
use crate::abstract_code_model::comment::Comment;
use std::collections::HashMap;
use std::any::{TypeId, Any};
use crate::abstract_code_model::{Entity, TypeName};

#[derive(Default, Debug)]
pub struct AbstractField<'a> {
    pub name: Option<Cow<'a, str>>,  // In tuple fields have no names
    pub type_name: Option<TypeName<'a>>,  // In enum fields may not have types.
    pub comment: Comment<'a>,
    pub inner_type: Option<Entity<'a>>,
    pub extensions: HashMap<String, Box<dyn Any>>  // For any and anyAttributes - vendor specific info
}

impl<'a> AbstractField<'a> {
    pub fn extension<T: Any>(&self, key: &str) -> Option<&T> {
        self
            .extensions
            .get(key)
            .and_then(|val| val.downcast_ref::<T>())
    }
}
