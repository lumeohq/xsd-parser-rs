use crate::abstract_code_model::comment::Comment;
use crate::xsd_model::Documentation;

impl<'a> Comment<'a> {
    pub fn parse(doc: &Documentation<'a>) -> Comment<'a> {
        Comment {
            text: if let Some(text) = doc.text { vec![text] } else { Vec::new() }
        }
    }

    pub fn parse_vec(docs: &[Documentation<'a>]) -> Comment<'a> {
        Comment {
            text: docs
            .iter()
            .filter(|x| x.text.is_some())
            .map(|x| x.text.unwrap())
            .collect()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::abstract_code_model::comment::Comment;
    use crate::xsd_model::Documentation;

    #[test]
    fn test_comment_parse() {
        let doc = Documentation {
            text: Some("A string"),
            ..Default::default()
        };
        let res = Comment::parse(&doc);
        assert_eq!(res.text.len(), 1);
        assert_eq!(res.text[0], "A string");
    }

    #[test]
    fn test_comment_parse_vec() {
        let docs = vec![
            Documentation {
                text: Some("A string"),
                ..Default::default()
            },
            Documentation {
                text: None,
                ..Default::default()
            },
            Documentation {
                text: Some("Another string"),
                ..Default::default()
            },
        ];

        let res = Comment::parse_vec(&docs);
        assert_eq!(res.text.len(), 2);
        assert_eq!(res.text[0], "A string");
        assert_eq!(res.text[1], "Another string");
    }
}
