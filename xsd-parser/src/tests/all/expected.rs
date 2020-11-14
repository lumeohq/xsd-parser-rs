#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct FooType {
    #[yaserde(prefix = "tns", rename = "A")]
    pub a: String,
    #[yaserde(prefix = "tns", rename = "B")]
    pub b: String,
}
