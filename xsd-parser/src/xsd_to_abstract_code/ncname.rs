use crate::abstract_code_model::ncname::NCName;
use crate::xsd_model::simple_types::ncname::NCName as XsdNCName;

impl<'a> NCName<'a> {
    pub fn parse(ncname: &XsdNCName<'a>) -> NCName<'a> {
        NCName( ncname.0 )
    }
}

#[cfg(test)]
mod test {
    use crate::abstract_code_model::ncname::NCName;
    use crate::xsd_model::simple_types::ncname::NCName as XsdNCName;

    #[test]
    fn test_ncname_parse() {
        let ncname = XsdNCName("A NCName");
        let res = NCName::parse(&ncname);
        assert_eq!(res.0, "A NCName");
    }
}
