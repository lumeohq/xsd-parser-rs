#[derive(Default, Clone, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct BarType(pub String);

#[derive(Default, Clone, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct BazType(pub i32);

#[derive(PartialEq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespaces = {"tns" = "http://example.com"})]
pub enum FooTypeChoice {
    Bar(BarType),
    Baz(BazType),
    __Unknown__(String),
}

impl Default for FooTypeChoice {
    fn default() -> FooTypeChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

#[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespaces = {"tns" = "http://example.com"})]
pub struct FooType {
    #[yaserde(flatten = true)]
    pub foo_type_choice: FooTypeChoice,
}
