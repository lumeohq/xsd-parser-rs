use crate::abstract_code_model::any_uri::AnyUri;

#[derive(Debug, Default)]
pub struct AppInfo<'a> {
    pub text: Option<&'a str>,
    pub source: Option<AnyUri<'a>>,
    // TODO: Vendor specific elements support.
    // TODO: Vendor specific attributes support.
}
