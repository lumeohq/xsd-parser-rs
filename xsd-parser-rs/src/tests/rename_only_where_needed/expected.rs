#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct FooType {
    #[yaserde(prefix = "tns")]
    pub min: i32,

    #[yaserde(prefix = "tns")]
    pub max: i32,
}
