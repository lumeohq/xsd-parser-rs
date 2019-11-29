
pub trait Validate {
    fn validate(&self) -> Result<(), &str>;
}

pub trait GenerateCode {
    fn generate_code(&self) -> String;
}




