use crate::generator::base::BaseGenerator;
use crate::generator::Generator2;
use crate::parser::types::Import;

pub trait ImportGenerator {
    fn generate(&self, entity: &Import, _gen: &Generator2) -> String {
        format!("//use {}  {};\n", entity.location, entity.name)
    }
}

pub struct DefaultImportGen;
impl ImportGenerator for DefaultImportGen {}
