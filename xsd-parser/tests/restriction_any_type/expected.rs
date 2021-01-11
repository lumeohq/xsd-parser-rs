// pub type AppSequence = AppSequenceType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespace = "tns: http://schemas.xmlsoap.org/ws/2005/04/discovery")]
pub struct AppSequenceType {
    #[yaserde(attribute, rename = "InstanceId")]
    pub instance_id: u32,

    #[yaserde(attribute, rename = "SequenceId")]
    pub sequence_id: Option<String>,

    #[yaserde(attribute, rename = "MessageNumber")]
    pub message_number: u32,
}

impl Validate for AppSequenceType {}
