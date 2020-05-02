use crate::abstract_code_model::any_uri::AnyUri;
use crate::xsd_model::simple_types::any_uri::AnyUri as XsdAnyUri;

impl<'a> AnyUri<'a> {
    pub fn parse(uri: &XsdAnyUri<'a>) -> AnyUri<'a> {
        AnyUri(uri.0)
    }
}

#[cfg(test)]
mod test {
    use crate::abstract_code_model::any_uri::AnyUri;
    use crate::xsd_model::simple_types::any_uri::AnyUri as XsdAnyUri;

    #[test]
    fn test_any_uri_parse() {
        let uri = XsdAnyUri("An URI");
        let res = AnyUri::parse(&uri);
        assert_eq!(res.0, "An URI");
    }
}
