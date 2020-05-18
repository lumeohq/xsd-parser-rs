use std::borrow::Cow;
use crate::abstract_code_model::comment::Comment;
use crate::abstract_code_model::Entity;
use std::collections::HashMap;
use std::any::{TypeId, Any};


#[derive(Default, Debug)]
pub struct Struct<'a> {
    pub name: Option<Cow<'a, str>>,
    pub comment: Comment<'a>,
    pub fields: Vec<StructField<'a>>,
    pub subtypes: Vec<Entity<'a>>,
    pub extensions: HashMap<TypeId, Box<dyn Any>>
}

impl<'a> Struct<'a> {
    pub fn extension<T: Any>(&self) -> Option<&T> {
        self
            .extensions
            .get(&TypeId::of::<T>())
            .and_then(|val| val.downcast_ref::<T>())
    }

    pub fn is_tuple(&self) -> bool {
        self.fields.iter().all(|f| f.name.is_some())
    }
}

#[derive(Default, Debug)]
pub struct StructField<'a> {
    pub name: Option<Cow<'a, str>>,
    pub type_name: Cow<'a, str>,
    pub comment: Comment<'a>,
    pub entities: Vec<Entity<'a>>,
    pub extensions: HashMap<TypeId, Box<dyn Any>>
}

impl<'a> StructField<'a> {
    pub fn extension<T: Any>(&self) -> Option<&T> {
        self
            .extensions
            .get(&TypeId::of::<T>())
            .and_then(|val| val.downcast_ref::<T>())
    }
}


#[cfg(test)]
mod test {
    use super::{StructField};
    use std::any::TypeId;

    pub struct TestType(pub i64);

    #[test]
    fn test() {
        let mut s = StructField::default();
        let t = TypeId::of::<TestType>();
        let w = Box::new(TestType(55));
        s.extensions.insert(t, w);
        assert_eq!(s.extension::<TestType>().unwrap().0, 55);
    }
}