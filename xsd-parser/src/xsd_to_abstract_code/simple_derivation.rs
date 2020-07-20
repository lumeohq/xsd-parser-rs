use crate::xsd_model::groups::simple_derivation::SimpleDerivation;
use crate::abstract_code_model::Entity;

pub fn simple_derivation_to_entity<'a>(value: &'a SimpleDerivation) -> Entity<'a> {
    match value {
        SimpleDerivation::List(list) => Entity::StructFields(vec![list.to_struct_field()]),
        SimpleDerivation::Restriction(val) => {}
        SimpleDerivation::Union(val) => {}
    }
}