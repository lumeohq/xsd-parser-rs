use roxmltree::Namespace;
use crate::parser::types::{File, TupleStruct, Struct, Enum, Alias, StructField, EnumCase};
use crate::parser::constants::attribute;
use crate::generator::MacroGenerator;
use std::borrow::Cow;

pub struct DefaultGenerator<'input> {
    target_ns: Option<Namespace<'input>>
}

impl<'input> DefaultGenerator<'input> {
    pub fn new(schema: &File<'input>) ->Self {
        DefaultGenerator{
            target_ns: schema.target_ns.clone(),
        }
    }
}

impl MacroGenerator for DefaultGenerator<'_> {
    fn tuple_struct_macro(&self, _: &TupleStruct) -> Cow<'static, str> {
        "#[derive(Default, PartialEq, Debug, UtilsTupleSerDe)]\n".into()
    }

    fn struct_macro(&self, _: &Struct) -> Cow<'static, str> {
        let derives = "#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]\n";
        match &self.target_ns {
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
        }.into()
    }

    fn enum_macro(&self, _: &Enum) -> Cow<'static, str> {
        "#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]".into()
    }

    fn alias_macro(&self, _: &Alias) -> Cow<'static, str> {
        "".into()
    }

    fn struct_field_macro(&self, sf: &StructField) -> Cow<'static, str> {
        unimplemented!()
    }

    fn enum_case_macro(&self, en: &EnumCase) -> Cow<'static, str> {
        unimplemented!()
    }
}