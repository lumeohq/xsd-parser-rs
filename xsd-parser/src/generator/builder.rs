use crate::generator::{
    alias::{AliasGenerator, DefaultAliasGen},
    base::{BaseGenerator, DefaultBaseGenerator},
    enum_case::{DefaultEnumCaseGen, EnumCaseGenerator},
    import::{DefaultImportGen, ImportGenerator},
    r#enum::{DefaultEnumGen, EnumGenerator},
    r#struct::{DefaultStructGen, StructGenerator},
    struct_field::{DefaultStructFieldGen, StructFieldGenerator},
    tuple_struct::{DefaultTupleStructGen, TupleStructGenerator},
    Generator,
};

#[derive(Default)]
pub struct GeneratorBuilder<'input> {
    gen: Generator<'input>,
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

    pub fn build(self) -> Generator<'input> {
        let mut gen = self.gen;
        gen.base.get_or_insert_with(|| Box::new(DefaultBaseGenerator {})); //.set_target_ns(&gen.target_ns);

        gen.tuple_struct_gen.get_or_insert_with(|| Box::new(DefaultTupleStructGen {}));

        gen.struct_gen.get_or_insert_with(|| Box::new(DefaultStructGen {}));

        gen.struct_field_gen.get_or_insert_with(|| Box::new(DefaultStructFieldGen {}));

        gen.enum_case_gen.get_or_insert_with(|| Box::new(DefaultEnumCaseGen {}));

        gen.enum_gen.get_or_insert_with(|| Box::new(DefaultEnumGen {}));

        gen.alias_gen.get_or_insert_with(|| Box::new(DefaultAliasGen {}));

        gen.import_gen.get_or_insert_with(|| Box::new(DefaultImportGen {}));

        gen
    }
}

#[cfg(test)]
mod test {
    use crate::{
        generator::{builder::GeneratorBuilder, tuple_struct::TupleStructGenerator, Generator},
        parser::types::{RsEntity, TupleStruct},
    };

    fn test_generator_state(gen: &Generator) {
        assert!(gen.tuple_struct_gen.is_some());
        assert!(gen.struct_gen.is_some());
        assert!(gen.struct_field_gen.is_some());
        assert!(gen.base.is_some());
        assert!(gen.enum_case_gen.is_some());
        assert!(gen.enum_gen.is_some());
        assert!(gen.alias_gen.is_some());
        assert!(gen.import_gen.is_some());
    }

    #[test]
    fn test_builder_default() {
        let gen = GeneratorBuilder::default().build();
        test_generator_state(&gen);
        assert!(gen.target_ns.borrow().is_none());
    }

    #[test]
    fn test_builder_with_custom_generators() {
        struct StubTupleStructGen;
        impl TupleStructGenerator for StubTupleStructGen {
            fn generate(&self, _: &TupleStruct, _: &Generator) -> String {
                "Tuple struct".into()
            }
        }

        let gen = GeneratorBuilder::default()
            .with_tuple_struct_gen(Box::new(StubTupleStructGen {}))
            .build();

        test_generator_state(&gen);

        let ts = RsEntity::TupleStruct(TupleStruct::default());
        assert_eq!(gen.generate(&ts), "Tuple struct");
    }
}
