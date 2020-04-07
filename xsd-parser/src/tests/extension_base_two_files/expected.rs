#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(
    prefix = "tns",
    namespace = "tns: http://example.com",
    namespace = "tns2: http://other.example.com"
)]
pub struct FooType {
    #[yaserde(prefix = "tns", rename = "a")]
    pub a: f64,

    #[yaserde(prefix = "tns2", rename = "b")]
    pub b: i32,

    #[yaserde(prefix = "tns2", rename = "c")]
    pub c: String,
}
