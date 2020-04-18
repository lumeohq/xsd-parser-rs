use crate::xsd_model::Documentation;
use crate::abstract_code_model::comment::Comment;

impl<'a> Comment<'a> {
    pub fn parse(doc: &Documentation<'a>) ->  Result<Comment<'a>, String> {
        let mut res = Comment::default();
        res.text = doc.text;
        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::xsd_model::Documentation;
    use crate::abstract_code_model::comment::Comment;

    #[test]
    fn test_comment_parse() {
        let doc = Documentation { text: Some("A string"), ..Default::default() };
        let res = Comment::parse(&doc).unwrap();
        assert_eq!(res.text.unwrap(), "A string");
    }
}
