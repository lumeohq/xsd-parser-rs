use roxmltree::Namespace;
use crate::parser::types::{File, TupleStruct, Struct, Enum, Alias, StructField, EnumCase, StructFieldSource};
use crate::parser::constants::attribute;
use crate::generator::Generator;
use std::borrow::Cow;

pub struct YaserdeGenerator<'input> {
    target_ns: Option<Namespace<'input>>
}

impl<'input> YaserdeGenerator<'input> {
    pub fn new(schema: &File<'input>) ->Self {
        YaserdeGenerator {
            target_ns: schema.target_ns.clone(),
        }
    }
}

impl<'input> Generator<'_> for YaserdeGenerator<'input> {
    fn target_ns(&self) -> &Option<Namespace<'_>> {
        &self.target_ns
    }

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

    fn struct_field_macro(&self, sf: &StructField) -> Cow<'static, str> {
        match sf.source {
            StructFieldSource::Attribute => yaserde_for_attribute(sf.name.as_str()).into(),
            StructFieldSource::Element => yaserde_for_element(sf.name.as_str(), self.target_ns.as_ref()).into(),
            _ => "".into()
        }
    }
}

fn yaserde_for_attribute(name: &str) -> String {
    if let Some(index) = name.find(':') {
        format!(
            "  #[yaserde(attribute, prefix = \"{}\" rename = \"{}\")]\n",
            &name[0..index],
            &name[index + 1..]
        )
    } else {
        format!("  #[yaserde(attribute, rename = \"{}\")]\n", name)
    }
}

pub fn yaserde_for_element(name: &str, target_namespace: Option<&roxmltree::Namespace>) -> String {
    let prefix = target_namespace.and_then(|ns| ns.name());
    match prefix {
        Some(p) => format!("  #[yaserde(prefix = \"{}\", rename = \"{}\")]\n", p, name),
        None => format!("  #[yaserde(rename = \"{}\")]\n", name),
    }
}