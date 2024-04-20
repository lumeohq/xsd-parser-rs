#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "http://pubchem.ncbi.nlm.nih.gov/pug_view")]
pub struct Section {
    #[yaserde(flatten)]
    pub section_choice: section::SectionChoice,

    #[yaserde(rename = "Description")]
    pub description: Option<String>,

    #[yaserde(rename = "URL")]
    pub url: Option<String>,
}

impl Validate for Section {}

pub mod section {
    use super::*;

    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(namespace = "http://pubchem.ncbi.nlm.nih.gov/pug_view")]

    pub enum SectionChoice {
        #[yaserde(rename = "TOCHeading")]
        Tocheading(String),
        #[yaserde(rename = "TOCID")]
        Tocid(i32),
        __Unknown__(String),
    }

    impl Default for SectionChoice {
        fn default() -> SectionChoice {
            Self::__Unknown__("No valid variants".into())
        }
    }

    impl Validate for SectionChoice {}
}
