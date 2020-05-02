use crate::abstract_code_model::annotation::Annotation;
use crate::abstract_code_model::any_uri::AnyUri;
use crate::abstract_code_model::id::Id;
use crate::abstract_code_model::ncname::NCName;
use crate::abstract_code_model::notation::Notation;
use crate::abstract_code_model::public::Public;
use crate::xsd_model::elements::notation::Notation as XsdNotation;

impl<'a> Notation<'a> {
    pub fn parse(notation: &XsdNotation<'a>) -> Notation<'a> {
        Notation {
            annotation: notation.annotation.as_ref().map(Annotation::parse),
            id: Id::parse(&notation.id),
            name: NCName::parse(&notation.name),
            public: notation.public.as_ref().map(Public::parse),
            system: notation.system.as_ref().map(AnyUri::parse),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::abstract_code_model::notation::Notation;
    use crate::xsd_model::elements::notation::Notation as XsdNotation;
    use crate::xsd_model::simple_types::any_uri::AnyUri as XsdAnyUri;
    use crate::xsd_model::simple_types::id::Id as XsdId;
    use crate::xsd_model::simple_types::ncname::NCName as XsdNCName;
    use crate::xsd_model::simple_types::token::Token as XsdToken;

    #[test]
    fn test_notation_parse() {
        let notation = XsdNotation {
            annotation: Some(Default::default()),
            id: Some(XsdId("An id")),
            name: XsdNCName("A name"),
            public: Some(XsdToken("Public")),
            system: Some(XsdAnyUri("An URI")),
            ..Default::default()
        };
        let res = Notation::parse(&notation);
        assert!(res.annotation.is_some());
        assert_eq!(res.id.0, Some("An id"));
        assert_eq!(res.name.0, "A name");
        assert_eq!(res.public.unwrap().0, "Public");
        assert_eq!(res.system.unwrap().0, "An URI");
    }
}
