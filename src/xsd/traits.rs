
pub trait Validate {
    fn validate(&self) -> Result<(), &str>;
}

pub trait GenerateCode {
    fn generate_code(&self) -> String;
}

pub trait GenerateYaserdeDerive {
    fn generate_yaserde_derive(&self) -> String {
        "#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]\n\
        #[yaserde(\n\
          prefix = \"unknown\",\n\
          namespace = \"unknown: unknown\"\n\
        )\n".to_string()
    }
}
