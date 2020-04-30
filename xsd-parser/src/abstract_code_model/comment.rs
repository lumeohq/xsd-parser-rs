#[derive(Debug, Default)]
pub struct Comment<'a> {
    pub text: Option<&'a str>,
}
