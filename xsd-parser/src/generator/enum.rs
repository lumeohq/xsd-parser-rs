use std::borrow::Cow;

use crate::{
    generator::{validator::gen_validate_impl, Generator},
    parser::types::{Enum, EnumSource},
};

pub trait EnumGenerator {
    fn generate(&self, entity: &Enum, gen: &Generator) -> String {
        let name = self.get_name(entity, gen);
        let default_case = format!(
            "impl Default for {name} {{\n\
            {indent}fn default() -> {name} {{\n\
            {indent}{indent}Self::__Unknown__(\"No valid variants\".into())\n\
            {indent}}}\n\
            }}",
            name = name,
            indent = gen.base().indent()
        );

        let str = format!(
            "{comment}{macros}\n\
            pub enum {name} {{\n\
                {cases}\n\
                {indent}__Unknown__({typename}),\n\
            }}\n\n\
            {default}\n\n\
            {validation}\n\n\
            {subtypes}\n\n",
            indent = gen.base().indent(),
            comment = self.format_comment(entity, gen),
            macros = self.macros(entity, gen),
            name = name,
            cases = self.cases(entity, gen),
            typename = self.get_type_name(entity, gen),
            default = default_case,
            subtypes = self.subtypes(entity, gen),
            validation = self.validation(entity, gen),
        );

        let name = self.get_type_name(entity, gen);
        let str2 = format!("impl YaSerialize for Box<{name}> {{\n\tfn serialize<W: Write>(&self, writer: &mut Serializer<W>) -> Result<(), String> {{\nself.serialize(writer)\n}}\nfn serialize_attributes(&self, attributes: Vec<OwnedAttribute>, namespace: Namespace) -> Result<(Vec<OwnedAttribute>, Namespace), String> {{\nself.serialize_attributes(attributes, namespace)\n}}\n}}\n\n");
        let str3 = format!("impl YaDeserialize for Box<{name}> {{\n\tfn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {{\n\t\tOk(Box::new({name}::deserialize(reader)?))}}\n}}\n\n");
        if name == "String" {
            str
        } else {
            str + str2.as_str() + str3.as_str()
        }
    }

    fn cases(&self, entity: &Enum, gen: &Generator) -> String {
        entity
            .cases
            .iter()
            .map(|case| gen.enum_case_gen().generate(case, gen))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn subtypes(&self, entity: &Enum, gen: &Generator) -> String {
        gen.base().join_subtypes(entity.subtypes.as_ref(), gen)
    }

    fn get_type_name(&self, entity: &Enum, gen: &Generator) -> String {
        gen.base().format_type_name(entity.type_name.as_str(), gen).into()
    }

    fn get_name(&self, entity: &Enum, gen: &Generator) -> String {
        gen.base().format_type_name(entity.name.as_str(), gen).into()
    }

    fn macros(&self, entity: &Enum, gen: &Generator) -> Cow<'static, str> {
        if entity.source == EnumSource::Union {
            return "#[derive(PartialEq, Debug, UtilsUnionSerDe)]".into();
        }

        let derives = "#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]";
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

    fn format_comment(&self, entity: &Enum, gen: &Generator) -> String {
        gen.base().format_comment(entity.comment.as_deref(), 0)
    }

    fn validation(&self, entity: &Enum, gen: &Generator) -> Cow<'static, str> {
        // Empty validation
        Cow::Owned(gen_validate_impl(self.get_name(entity, gen).as_str(), ""))
    }
}

pub struct DefaultEnumGen;
impl EnumGenerator for DefaultEnumGen {}
