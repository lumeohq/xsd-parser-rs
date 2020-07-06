use crate::abstract_code_model::Entity;

// It can be a directory, file, namespace, etc.
#[derive(Default, Debug)]
pub struct Module<'a> {
    pub name: &'a str,
    pub entities: Vec<Entity<'a>>,
    pub private: bool
}


