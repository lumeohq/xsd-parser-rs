#[derive(PartialEq, Debug, UtilsUnionSerDe)]
pub enum FooType {
    Int(i32),
    String(String),
    __Unknown__(String),
}

impl Default for FooType {
    fn default() -> FooType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for FooType {}
