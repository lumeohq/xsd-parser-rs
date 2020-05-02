use crate::abstract_code_model::any_uri::AnyUri;
use crate::abstract_code_model::app_info::AppInfo;
use crate::xsd_model::elements::app_info::AppInfo as XsdAppInfo;

impl<'a> AppInfo<'a> {
    pub fn parse(app_info: &XsdAppInfo<'a>) -> AppInfo<'a> {
        AppInfo {
            text: app_info.text,
            source: app_info.source.as_ref().map(|x| AnyUri::parse(x)),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::abstract_code_model::any_uri::AnyUri;
    use crate::abstract_code_model::app_info::AppInfo;
    use crate::xsd_model::elements::app_info::AppInfo as XsdAppInfo;
    use crate::xsd_model::simple_types::any_uri::AnyUri as XsdAnyUri;

    #[test]
    fn test_app_info_parse() {
        let app_info = XsdAppInfo {
            text: Some("A string"),
            source: Some(XsdAnyUri("An URI")),
            ..Default::default()
        };
        let res = AppInfo::parse(&app_info);
        assert_eq!(res.text.unwrap(), "A string");
        assert_eq!(res.source.unwrap(), AnyUri("An URI"));
    }
}
