pub mod alias;
pub mod base;
pub mod builder;
pub mod default;
pub mod r#enum;
pub mod enum_case;
pub mod import;
pub mod r#struct;
pub mod struct_field;
pub mod tuple_struct;
mod utils;
pub mod validator;

use std::{borrow::Borrow, cell::RefCell};

use roxmltree::Namespace;

use crate::{
    generator::{
        alias::AliasGenerator, base::BaseGenerator, enum_case::EnumCaseGenerator,
        import::ImportGenerator, r#enum::EnumGenerator, r#struct::StructGenerator,
        struct_field::StructFieldGenerator, tuple_struct::TupleStructGenerator,
    },
    parser::types::{RsEntity, RsFile},
};

#[derive(Default)]
pub struct Generator<'input> {
    pub target_ns: RefCell<Option<Namespace<'input>>>,
    pub xsd_ns: RefCell<Option<Namespace<'input>>>,

    pub tuple_struct_gen: Option<Box<dyn TupleStructGenerator>>,
    pub struct_gen: Option<Box<dyn StructGenerator>>,
    pub struct_field_gen: Option<Box<dyn StructFieldGenerator>>,
    pub base: Option<Box<dyn BaseGenerator>>,
    pub enum_case_gen: Option<Box<dyn EnumCaseGenerator>>,
    pub enum_gen: Option<Box<dyn EnumGenerator>>,
    pub alias_gen: Option<Box<dyn AliasGenerator>>,
    pub import_gen: Option<Box<dyn ImportGenerator>>,
}

impl<'input> Generator<'input> {
    pub fn generate_rs_file(&self, schema: &RsFile<'input>) -> String {
        *self.target_ns.borrow_mut() = schema.target_ns.clone();
        *self.xsd_ns.borrow_mut() = schema.xsd_ns.clone();
        schema.types.iter().map(|entity| self.generate(entity)).collect()
    }

    pub fn generate(&self, entity: &RsEntity) -> String {
        use RsEntity::*;
        match entity {
            TupleStruct(ts) => self.tuple_struct_gen.as_ref().unwrap().generate(ts, self),
            Struct(st) => self.struct_gen.as_ref().unwrap().generate(st, self),
            StructField(sf) => self.struct_field_gen().generate(sf, self),
            Enum(en) => self.enum_gen.as_ref().unwrap().generate(en, self),
            EnumCase(ec) => self.enum_case_gen().generate(ec, self),
            Alias(al) => self.alias_gen.as_ref().unwrap().generate(al, self),
            Import(im) => self.import_gen.as_ref().unwrap().generate(im, self),
        }
    }

    pub fn base(&self) -> &dyn BaseGenerator {
        self.base.as_ref().unwrap().borrow()
    }

    pub fn struct_field_gen(&self) -> &dyn StructFieldGenerator {
        self.struct_field_gen.as_ref().unwrap().borrow()
    }

    pub fn enum_case_gen(&self) -> &dyn EnumCaseGenerator {
        self.enum_case_gen.as_ref().unwrap().borrow()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        generator::builder::GeneratorBuilder,
        parser::types::{RsEntity, RsFile, TupleStruct},
    };

    #[test]
    fn test_generate_rs_file() {
        let gen = GeneratorBuilder::default().build();
        let mut rs_file = RsFile::default();
        assert!(gen.generate_rs_file(&rs_file).is_empty());

        rs_file.types.push(RsEntity::TupleStruct(TupleStruct {
            name: "name".to_string(),
            comment: Some("comment".into()),
            type_name: "type".to_string(),
            ..Default::default()
        }));
        let comment = "// comment\n";
        let macros = "#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]\n";
        let validation = "impl Validate for Name {}\n";
        let expected =
            format!("{}{}pub struct Name (pub Type);\n\n{}", comment, macros, validation);
        assert_eq!(gen.generate_rs_file(&rs_file), expected);
    }
}
