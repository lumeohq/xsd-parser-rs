#[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespaces = {"tns" = "http://example.com"})]
pub struct FooType {
    #[yaserde(prefix = "tns", rename = "Extension")]
    pub extension: foo_type::ExtensionType,
}

impl Validate for FooType {}

pub mod foo_type {
    use super::*;

    #[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "tns", namespaces = {"tns" = "http://example.com"})]
    pub struct ExtensionType {}

    impl Validate for ExtensionType {}
}

#[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespaces = {"tns" = "http://example.com"})]
pub struct BarType {
    #[yaserde(prefix = "tns", rename = "Extension")]
    pub extension: bar_type::ExtensionType,
}

impl Validate for BarType {}

pub mod bar_type {
    use super::*;

    #[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "tns", namespaces = {"tns" = "http://example.com"})]
    pub struct ExtensionType {}

    impl Validate for ExtensionType {}
}

