use crate::generator::alias::{AliasGenerator, DefaultAliasGen};
use crate::generator::base::{BaseGenerator, DefaultBaseGenerator};
use crate::generator::enum_case::{DefaultEnumCaseGen, EnumCaseGenerator};
use crate::generator::import::{DefaultImportGen, ImportGenerator};
use crate::generator::r#enum::{DefaultEnumGen, EnumGenerator};
use crate::generator::r#struct::{DefaultStructGen, StructGenerator};
use crate::generator::struct_field::{DefaultStructFieldGen, StructFieldGenerator};
use crate::generator::tuple_struct::{DefaultTupleStructGen, TupleStructGenerator};
use crate::generator::Generator2;

#[derive(Default)]
pub struct GeneratorBuilder<'input> {
    gen: Generator2<'input>,
}

#[allow(dead_code)]
impl<'input> GeneratorBuilder<'input> {
    pub fn with_base_gen(mut self, base: Box<dyn BaseGenerator>) -> Self {
        self.gen.base = Some(base);
        self
    }

    pub fn with_tuple_struct_gen(mut self, tsg: Box<dyn TupleStructGenerator>) -> Self {
        self.gen.tuple_struct_gen = Some(tsg);
        self
    }

    pub fn with_struct_gen(mut self, sg: Box<dyn StructGenerator>) -> Self {
        self.gen.struct_gen = Some(sg);
        self
    }

    pub fn with_struct_field_gen(mut self, sfg: Box<dyn StructFieldGenerator>) -> Self {
        self.gen.struct_field_gen = Some(sfg);
        self
    }

    pub fn with_enum_case_gen(mut self, ecg: Box<dyn EnumCaseGenerator>) -> Self {
        self.gen.enum_case_gen = Some(ecg);
        self
    }

    pub fn with_enum_gen(mut self, eg: Box<dyn EnumGenerator>) -> Self {
        self.gen.enum_gen = Some(eg);
        self
    }

    pub fn with_alias_gen(mut self, al: Box<dyn AliasGenerator>) -> Self {
        self.gen.alias_gen = Some(al);
        self
    }

    pub fn with_import_gen(mut self, im: Box<dyn ImportGenerator>) -> Self {
        self.gen.import_gen = Some(im);
        self
    }

    pub fn build(self) -> Generator2<'input> {
        let mut gen = self.gen;
        gen.base
            .get_or_insert_with(|| Box::new(DefaultBaseGenerator {})); //.set_target_ns(&gen.target_ns);

        gen.tuple_struct_gen
            .get_or_insert_with(|| Box::new(DefaultTupleStructGen {}));

        gen.struct_gen
            .get_or_insert_with(|| Box::new(DefaultStructGen {}));

        gen.struct_field_gen
            .get_or_insert_with(|| Box::new(DefaultStructFieldGen {}));

        gen.enum_case_gen
            .get_or_insert_with(|| Box::new(DefaultEnumCaseGen {}));

        gen.enum_gen
            .get_or_insert_with(|| Box::new(DefaultEnumGen {}));

        gen.alias_gen
            .get_or_insert_with(|| Box::new(DefaultAliasGen {}));

        gen.import_gen
            .get_or_insert_with(|| Box::new(DefaultImportGen {}));

        gen
    }
}
