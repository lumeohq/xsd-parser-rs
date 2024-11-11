// pub type AppSequence = AppSequenceType;
#[derive(Default, Clone, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(prefix = "tns", namespaces = {"tns" =  "http://schemas.xmlsoap.org/ws/2005/04/discovery"})]
pub struct AppSequenceType {
    #[yaserde(attribute = true, rename = "InstanceId")]
    pub instance_id: u32,

    #[yaserde(attribute = true, rename = "SequenceId")]
    pub sequence_id: Option<String>,

    #[yaserde(attribute = true, rename = "MessageNumber")]
    pub message_number: u32,
}

impl Validate for AppSequenceType {}
