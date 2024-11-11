#[derive(PartialEq, Debug, Clone, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespaces = {"tns" = "http://example.com"})]
pub enum FooType {
    #[yaserde(rename = "OFF")]
    Off,
    #[yaserde(rename = "ON")]
    On,
    #[yaserde(rename = "AUTO")]
    Auto,
    __Unknown__(String),
}

impl Default for FooType {
    fn default() -> FooType {
        Self::__Unknown__("No valid variants".into())
    }
}
impl Validate for FooType {}


#[derive(Default, Clone, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct FooType2(pub String);

impl Validate for FooType2 {}

