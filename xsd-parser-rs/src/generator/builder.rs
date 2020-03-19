use crate::generator::base::{BaseGenerator, DefaultBaseGenerator};
use crate::generator::enum_case::{EnumCaseGenerator, DefaultEnumCaseGen};
use crate::generator::r#struct::{DefaultStructGen, StructGenerator};
use crate::generator::struct_field::{StructFieldGenerator, DefaultStructFieldGen};
use crate::generator::tuple_struct::{TupleStructGenerator, DefaultTupleStructGen};
use crate::generator::Generator2;

#[derive(Default)]
pub struct GeneratorBuilder<'input> {
    gen: Generator2<'input>,
}

impl<'input> GeneratorBuilder<'input> {
    pub fn new(gen: Generator2<'input>) -> Self {
        Self { gen }
    }
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

    pub fn build(mut self) -> Generator2<'input> {
        let mut gen = self.gen;
        gen.base
            .get_or_insert_with(|| Box::new(DefaultBaseGenerator { target_ns: None })); //.set_target_ns(&gen.target_ns);

        gen.tuple_struct_gen
            .get_or_insert_with(|| Box::new(DefaultTupleStructGen {}));

        gen.struct_gen
            .get_or_insert_with(|| Box::new(DefaultStructGen {}));

        gen.struct_field_gen
            .get_or_insert_with(|| Box::new(DefaultStructFieldGen {}));

        gen.enum_case_gen
            .get_or_insert_with(|| Box::new(DefaultEnumCaseGen {}));

        gen
    }
}
