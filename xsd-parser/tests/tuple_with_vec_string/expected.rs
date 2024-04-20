#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct FooType(pub Vec<String>);

impl Validate for FooType {}
// pub type Foo = FooType;
