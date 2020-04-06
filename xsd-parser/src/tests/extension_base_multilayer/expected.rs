#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct BarType {
    #[yaserde(prefix = "tns", rename = "aa")]
    pub aa: i32,

    #[yaserde(prefix = "tns", rename = "bb")]
    pub bb: String,
}

impl Validate for BarType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct FooType {
    #[yaserde(prefix = "tns", rename = "Messages")]
    pub messages: foo_type::MessagesType,
}

impl Validate for FooType {}

pub mod foo_type {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
    pub struct MessagesType {
        #[yaserde(prefix = "tns", rename = "a")]
        pub a: String,

        #[yaserde(prefix = "tns", rename = "aa")]
        pub aa: i32,

        #[yaserde(prefix = "tns", rename = "bb")]
        pub bb: String,
    }

    impl Validate for MessagesType {}
}
