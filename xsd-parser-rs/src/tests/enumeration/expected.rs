#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
pub enum FooType {
    #[yaserde(rename = "OFF")]
    Off,

    #[yaserde(rename = "ON")]
    On,

    #[yaserde(rename = "AUTO")]
    Auto,

    __Unknown__(String)
}

impl Default for FooType {
    fn default() -> FooType {
        Self::__Unknown__("No valid variants".into())
    }
}
