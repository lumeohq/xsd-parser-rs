#[derive(Default, Clone, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Id(pub String);

impl Validate for Id {}
#[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespaces = {"tns" = "http://example.com"})]
pub struct FooType {
    #[yaserde(attribute = true, prefix = "tns", rename = "id")]
    pub id: Option<Id>,
}

impl Validate for FooType {}
