// TODO: think how it can actually be generated

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct FooType {
    #[yaserde(attribute, rename = "b")]
    pub b: Option<String>,

    #[yaserde(attribute, rename = "a")]
    pub a: Option<String>,
}
