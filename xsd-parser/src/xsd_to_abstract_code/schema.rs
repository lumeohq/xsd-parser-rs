use crate::xsd_model::Schema;
use crate::abstract_code_model::module::Module;

mod tt {
    include!("enumeration.rs");
}


impl<'a> Schema<'a> {
    pub fn to_module(&self, name: &'a str) -> Module {
        for include in self.includes.as_slice() {

        }
        Module {
            name,
            entities: vec![],
            private: false
        }
    }
}