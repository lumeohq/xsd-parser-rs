use crate::xsd_model::Schema;
use crate::abstract_code_model::module::Module;

impl<'a> Schema<'a> {
    pub fn to_module(&self, name: &'a str) -> Module {
        Module {
            name,
            entities: vec![],
            private: false
        }
    }
}