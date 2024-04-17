use crate::{generator::Generator, parser::types::Import};

pub trait ImportGenerator {
    fn generate(&self, entity: &Import, _gen: &Generator) -> String {
        format!("//use {}  {};\n", entity.location, entity.name)
    }
}

pub struct DefaultImportGen;
impl ImportGenerator for DefaultImportGen {}
