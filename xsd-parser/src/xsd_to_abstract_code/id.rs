use crate::abstract_code_model::id::Id;
use crate::xsd_model::simple_types::Id as OptionalXsdId;

impl<'a> Id<'a> {
    pub fn parse(id: &OptionalXsdId<'a>) -> Id<'a> {
        if let Some(id) = id {
            Id(Some(id.0))
        } else {
            Id(None)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::abstract_code_model::id::Id;
    use crate::xsd_model::simple_types::Id as OptionalXsdId;
    use crate::xsd_model::simple_types::id::Id as XsdId;

    #[test]
    fn test_id_parse() {
        let id : OptionalXsdId = Some( XsdId("An id") );
        let res = Id::parse(&id);
        assert_eq!(res.0, Some("An id"));
    }
}
