#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct FooType(pub Vec<i32>);

impl Validate for FooType {}
// pub type Foo = FooType;
