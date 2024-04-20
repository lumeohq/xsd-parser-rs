#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct FooType {
    #[yaserde(prefix = "tns", rename = "Name")]
    pub name: String,
}

impl Validate for FooType {}
// pub type Foo = FooType;
