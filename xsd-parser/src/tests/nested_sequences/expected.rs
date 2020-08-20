#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde()]
pub struct Foo {
    #[yaserde(rename = "bar")]
    pub bar: xs::Date,

    #[yaserde(rename = "Bazs")]
    pub bazs: foo::BazsType,
}

impl Validate for Foo {}

pub mod foo {
    use super::*;

    #[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde()]
    pub struct BazsType {
        #[yaserde(rename = "initialsBaz")]
        pub initials_baz: bazs_type::InitialsBazType,
    }

    impl Validate for BazsType {}

    pub mod bazs_type {
        use super::*;

        #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
        pub enum InitialsBazType {
            Fizz01,
            Fizz02,
            Fizz03,
            Fizz04,
            Fizz05,
            Fizz06,
            Fizz07,
            Fizz08,
            Fizz09,
            Fizz10,
            Fizz11,
            Fizz12,
            Fizz13,
            Fizz14,
            Fizz15,
            __Unknown__(String),
        }

        impl Default for InitialsBazType {
            fn default() -> InitialsBazType {
                Self::__Unknown__("No valid variants".into())
            }
        }

        impl Validate for InitialsBazType {}
    }


}


