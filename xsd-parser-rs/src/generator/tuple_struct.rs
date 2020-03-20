use crate::generator::base::BaseGenerator;
use crate::generator::validator::{gen_facet_validation, gen_validate_impl};
use crate::generator::Generator2;
use crate::parser::types::{RsEntity, TupleStruct};
use std::borrow::Cow;

pub trait TupleStructGenerator {
    fn generate(&self, entity: &TupleStruct, gen: &Generator2) -> String {
        format!(
            "{comment}{macros}pub struct {name} (pub {typename});\n{subtypes}\n{validation}\n",
            comment = self.format_comment(entity, gen),
            name = self.get_name(entity, gen),
            macros = self.macros(entity, gen),
            typename = self.get_type_name(entity, gen),
            subtypes = self.subtypes(entity, gen),
            validation = self.validation(entity, gen),
        )
    }

    fn subtypes(&self, entity: &TupleStruct, gen: &Generator2) -> String {
        gen.base().join_subtypes(entity.subtypes.as_ref(), gen)
    }

    fn get_type_name(&self, entity: &TupleStruct, gen: &Generator2) -> String {
        gen.base().modify_type(
            gen.base().format_type_name(entity.type_name.as_str(), gen).as_ref(),
            &entity.type_modifiers,
        )
        .into()
    }

    fn get_name(&self, entity: &TupleStruct, gen: &Generator2) -> String {
        gen
            .base()
            .format_type_name(entity.name.as_str(), gen)
            .into()
    }

    fn macros(&self, _entity: &TupleStruct, _gen: &Generator2) -> Cow<'static, str> {
        "#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]\n".into()
    }

    fn format_comment(&self, entity: &TupleStruct, gen: &Generator2) -> String {
        gen.base().format_comment(entity.comment.as_deref(), 0)
    }

    fn validation(&self, entity: &TupleStruct, gen: &Generator2) -> Cow<'static, str> {
        let body = entity
            .facets
            .iter()
            .map(|f| gen_facet_validation(&f.facet_type, "0"))
            .fold(String::new(), |x, y| (x + &y));
        Cow::Owned(gen_validate_impl(self.get_name(entity, gen).as_str(), body.as_str()))
    }
}

pub struct DefaultTupleStructGen;
impl TupleStructGenerator for DefaultTupleStructGen {}
