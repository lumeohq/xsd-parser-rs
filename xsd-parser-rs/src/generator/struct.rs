use crate::parser::types::{Struct, RsEntity, StructField};
use crate::generator::Generator2;
use crate::generator::base::BaseGenerator;
use std::borrow::Cow;
use crate::generator::validator::{gen_facet_validation, gen_validate_impl};

pub trait StructGen {
    fn generate(&self, entity: &Struct, gen: &Generator2) -> String {
        let base = gen.base();
        let name = self.get_type_name(entity, base);

        format!(
            "{comment}{macros}pub struct {name} {{{fields}}}\n\n{validation}\n{subtypes}\n{fields_subtypes}",
            comment = self.format_comment(entity.comment.as_deref(), base),
            macros = self.macros(entity),
            name = name,
            fields = self.fields(entity, gen),
            subtypes = self.subtypes(entity.subtypes.as_ref(), gen),
            fields_subtypes = self.fields_subtypes(entity, gen),
            validation = self.validation(entity, name.as_ref()),
        )
    }

    fn fields(&self, entity: &Struct, gen: &Generator2) -> String {
        let fields = entity
            .fields
            .borrow()
            .iter()
            .map(|f| gen.struct_field_gen().generate(f, gen))
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>()
            .join("\n\n");

        if fields.is_empty() {
            fields
        } else {
            format!("\n{}\n", fields)
        }
    }

    fn subtypes(&self, subtypes: &[RsEntity], gen: &Generator2) -> String {
        subtypes
            .iter()
            .map(|f| gen.generate(f))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn fields_subtypes(&self, entity: &Struct, gen: &Generator2) -> String {
        entity
            .fields
            .borrow()
            .iter()
            .map(|f| self.subtypes(f.subtypes.as_ref(), gen))
            .collect::<Vec<String>>()
            .join("")
    }

    fn get_type_name(&self, entity: &Struct, base: &Box<dyn BaseGenerator>) -> String {
        base.format_type_name(entity.name.as_str()).into()
    }

    fn macros(&self, _: &Struct) -> Cow<'static, str> { "".into() }

    fn format_comment(&self, comment: Option<&str>, base: &Box<dyn BaseGenerator>) -> String {
        base.format_comment(comment, base.indent_size())
    }

    fn validation(&self, entity: &Struct, formatted_type_name: &str) -> Cow<'static, str> {
        // Empty validation
        Cow::Owned(gen_validate_impl(
            formatted_type_name,
            "",
        ))
    }
}

pub struct DefaultStructGen;
impl StructGen for DefaultStructGen{}