use crate::abstract_code_model::Entity;

// It can be a directory, file, namespace, etc.
#[derive(Default, Debug)]
pub struct Module<'a> {
    name: String,
    entities: Vec<Entity<'a>>,
    private: bool
}


