#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct FooType(pub String);

impl Validate for FooType {}
// pub type Foo = FooType;
