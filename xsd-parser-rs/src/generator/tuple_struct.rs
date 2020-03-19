use crate::parser::types::{TupleStruct, RsEntity};
use crate::generator::Generator2;
use std::borrow::Cow;
use crate::generator::base::BaseGenerator;
use crate::generator::validator::{gen_facet_validation, gen_validate_impl};


pub trait TupleStructGenerator {
    fn generate(&self, entity: &TupleStruct, gen: &Generator2) -> String {
        let base = gen.base();
        let type_name = self.get_type_name(entity, base);
        format!(
            "{comment}{macros}pub struct {name} (pub {typename});\n{subtypes}\n{validation}\n",
            comment = self.format_comment(entity.comment.as_deref(), base),
            name = base.format_type_name(entity.name.as_str()),
            macros = self.macros(entity),
            typename = type_name,
            subtypes = self.subtypes(entity.subtypes.as_ref(), gen),
            validation = self.validation(entity, type_name.as_str()),
        )
    }

    fn subtypes(&self, subtypes: &[RsEntity], gen: &Generator2) -> String {
        subtypes
            .iter()
            .map(|f| gen.generate(f))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn get_type_name(&self, entity: &TupleStruct, base: &Box<dyn BaseGenerator>) -> String {
        base.modify_type(
            base.format_type_name(entity.type_name.as_str()).as_ref(),
            &entity.type_modifiers
        ).into()
    }

    fn macros(&self, _: &TupleStruct) -> Cow<'static, str> { "".into() }

    fn format_comment(&self, comment: Option<&str>, base: &Box<dyn BaseGenerator>) -> String {
        base.format_comment(comment, base.indent_size())
    }

    fn validation(&self, entity: &TupleStruct, formatted_type_name: &str) -> Cow<'static, str> {
        let body = entity
            .facets
            .iter()
            .map(|f| gen_facet_validation(&f.facet_type, "0"))
            .fold(String::new(), |x, y| (x + &y));
        Cow::Owned(gen_validate_impl(
            formatted_type_name,
            body.as_str(),
        ))
    }
}

pub struct DefaultTupleStructGen;
impl TupleStructGenerator for DefaultTupleStructGen{}

