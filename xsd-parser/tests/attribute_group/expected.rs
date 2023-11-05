#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://example.com")]
pub struct FooType {
    #[yaserde(attribute, rename = "id")]
    pub id: Option<String>,

    #[yaserde(attribute, rename = "aNumber")]
    pub a_number: Option<i32>,
}

impl Validate for FooType {}


// pub type Foo = FooType;
