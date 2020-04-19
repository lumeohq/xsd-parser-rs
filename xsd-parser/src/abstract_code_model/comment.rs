#[derive(Default, Debug)]
pub struct Comment<'a> {
    pub text: Option<&'a str>,
}
