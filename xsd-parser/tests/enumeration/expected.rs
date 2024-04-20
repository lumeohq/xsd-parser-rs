#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
pub enum FooType {
    OFF,
    ON,
    AUTO,
    __Unknown__(String),
}

impl Default for FooType {
    fn default() -> FooType {
        Self::__Unknown__("No valid variants".into())
    }
}
impl Validate for FooType {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
pub enum FooType1 {
    OFF,
    ON,
    On,
    __Unknown__(String),
}

impl Default for FooType1 {
    fn default() -> FooType1 {
        Self::__Unknown__("No valid variants".into())
    }
}
impl Validate for FooType1 {}

#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct FooType2(pub String);

impl Validate for FooType2 {}
