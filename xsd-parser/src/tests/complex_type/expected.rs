#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct FooType {
    #[yaserde(prefix = "tns", rename = "Min")]
    pub min: i32,

    #[yaserde(prefix = "tns", rename = "Max")]
    pub max: i32,
}

impl Validate for FooType {}
