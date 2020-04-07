use crate::generator::Generator;
use crate::parser::types::Import;

pub trait ImportGenerator {
    fn generate(&self, entity: &Import, _gen: &Generator) -> String {
        format!("//use {}  {};\n", entity.location, entity.name)
    }
}

pub struct DefaultImportGen;
impl ImportGenerator for DefaultImportGen {}
