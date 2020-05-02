use crate::abstract_code_model::public::Public;
use crate::xsd_model::simple_types::public::Public as XsdPublic;

impl<'a> Public<'a> {
    pub fn parse(ncname: &XsdPublic<'a>) -> Public<'a> {
        Public(ncname.0)
    }
}

#[cfg(test)]
mod test {
    use crate::abstract_code_model::public::Public;
    use crate::xsd_model::simple_types::token::Token;

    #[test]
    fn test_public_parse() {
        let public = Token("Public");
        let res = Public::parse(&public);
        assert_eq!(res.0, "Public");
    }
}
