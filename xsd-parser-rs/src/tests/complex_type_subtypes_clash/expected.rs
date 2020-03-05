pub mod foo_type {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
    pub struct Extension {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct FooType {
    #[yaserde(prefix = "tns", rename = "Min")]
    pub extension: foo_type::Extension,
}

pub mod bar_type {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
    pub struct Extension {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct BarType {
    #[yaserde(prefix = "tns", rename = "Min")]
    pub extension: bar_type::Extension,
}
