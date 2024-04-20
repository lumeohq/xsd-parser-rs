#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug, UtilsUnionSerDe)]
pub enum FooType {
    int(i32),
    string(String),
    __Unknown__(String),
}

impl Default for FooType {
    fn default() -> FooType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for FooType {}
