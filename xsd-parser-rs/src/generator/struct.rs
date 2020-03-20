use crate::generator::base::BaseGenerator;
use crate::generator::validator::gen_validate_impl;
use crate::generator::Generator;
use crate::parser::types::Struct;
use std::borrow::{Borrow, Cow};

pub trait StructGenerator {
    fn generate(&self, entity: &Struct, gen: &Generator) -> String {
        format!(
            "{comment}{macros}pub struct {name} {{{fields}}}\n\n{validation}\n{subtypes}\n{fields_subtypes}",
            comment = self.format_comment(entity, gen),
            macros = self.macros(entity, gen),
            name = self.get_type_name(entity, gen),
            fields = self.fields(entity, gen),
            subtypes = self.subtypes(entity, gen),
            fields_subtypes = self.fields_subtypes(entity, gen),
            validation = self.validation(entity, gen),
        )
    }

    fn fields(&self, entity: &Struct, gen: &Generator) -> String {
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

    fn subtypes(&self, entity: &Struct, gen: &Generator) -> String {
        gen.base().join_subtypes(entity.subtypes.as_ref(), gen)
    }

    fn fields_subtypes(&self, entity: &Struct, gen: &Generator) -> String {
        entity
            .fields
            .borrow()
            .iter()
            .map(|f| gen.base().join_subtypes(f.subtypes.as_ref(), gen))
            .collect::<Vec<String>>()
            .join("")
    }

    fn get_type_name(&self, entity: &Struct, gen: &Generator) -> String {
        gen.base()
            .format_type_name(entity.name.as_str(), gen)
            .into()
    }

    fn macros(&self, _entity: &Struct, gen: &Generator) -> Cow<'static, str> {
        let derives = "#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]\n";
        let tns = gen.target_ns.borrow();
        match tns.as_ref() {
            Some(tn) => match tn.name() {
                Some(name) => format!(
                    "{derives}#[yaserde(prefix = \"{prefix}\", namespace = \"{prefix}: {uri}\")]\n",
                    derives = derives,
                    prefix = name,
                    uri = tn.uri()
                ),
                None => format!(
                    "{derives}#[yaserde(namespace = \"{uri}\")]\n",
                    derives = derives,
                    uri = tn.uri()
                ),
            },
            None => format!("{derives}#[yaserde()]\n", derives = derives),
        }
        .into()
    }

    fn format_comment(&self, entity: &Struct, gen: &Generator) -> String {
        gen.base().format_comment(entity.comment.as_deref(), 0)
    }

    fn validation(&self, entity: &Struct, gen: &Generator) -> Cow<'static, str> {
        // Empty validation
        Cow::Owned(gen_validate_impl(
            self.get_type_name(entity, gen).as_str(),
            "",
        ))
    }
}

pub struct DefaultStructGen;
impl StructGenerator for DefaultStructGen {}
