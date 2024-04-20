#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct FooType(pub xs::Integer);

impl Validate for FooType {}
// pub type Foo = FooType;
