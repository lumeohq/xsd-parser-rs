#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct BarType {
    #[yaserde(prefix = "tns", rename = "b")]
    pub b: i32,

    #[yaserde(prefix = "tns", rename = "c")]
    pub c: String,
}

impl Validate for BarType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct FooType {
    #[yaserde(prefix = "tns", rename = "a")]
    pub a: f64,

    #[yaserde(prefix = "tns", rename = "b")]
    pub b: i32,

    #[yaserde(prefix = "tns", rename = "c")]
    pub c: String,
}

impl Validate for FooType {}
