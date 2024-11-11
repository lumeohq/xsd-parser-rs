#[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespaces = {"tns" = "http://example.com"})]
pub struct BarType {
    #[yaserde(attribute = true, rename = "a")]
    pub a: Option<String>,
}

impl Validate for BarType {}

#[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespaces = {"tns" = "http://example.com"})]
pub struct FooType {
    #[yaserde(prefix = "tns", rename = "Bar")]
    pub bar: foo_type::BarType,
}

impl Validate for FooType {}

pub mod foo_type {
    use super::*;

    #[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "tns", namespaces = {"tns" = "http://example.com"})]
    pub struct BarType {
        #[yaserde(attribute = true, rename = "b")]
        pub b: Option<String>,

        #[yaserde(attribute = true, rename = "a")]
        pub a: Option<String>,
    }

    impl Validate for BarType {}
}
