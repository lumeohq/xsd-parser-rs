use crate::abstract_code_model::comment::Comment;
use crate::xsd_model::Annotation;
use crate::xsd_model::AppInfo;
use crate::xsd_model::Documentation;
use crate::xsd_model::Notation;

pub struct S{}

impl<'a> Comment<'a> {
    pub fn from_documentation(doc: &Documentation<'a>) -> Comment<'a> {
        Comment::from_opt_str(doc.text)
    }

    pub fn from_app_info(app_info: &AppInfo<'a>) -> Comment<'a> {
        Comment::from_opt_str(app_info.text)
    }

    pub fn from_annotation(annotation: &Annotation<'a>) -> Comment<'a> {
        let app_info_comment = Comment::from_app_info_vec(&annotation.app_infos);
        let documentation_comment = Comment::from_documentation_vec(&annotation.documentations);
        Comment::from_comment_vec(&[app_info_comment, documentation_comment])
    }

    pub fn from_opt_annotation(annotation: &Option<Annotation<'a>>) -> Comment<'a> {
        if let Some(annotation) = annotation {
            Comment::from_annotation(annotation)
        } else {
            Comment::default()
        }
    }

    pub fn from_notation(notation: &Notation<'a>) -> Comment<'a> {
        if let Some(annotation) = &notation.annotation {
            Comment::from_annotation(&annotation)
        } else {
            Comment { text: Vec::new() }
        }
    }

    fn from_opt_str(s: Option<&'a str>) -> Comment<'a> {
        Comment {
            text: if let Some(text) = s {
                vec![text]
            } else {
                Vec::new()
            },
        }
    }

    fn from_app_info_vec(docs: &[AppInfo<'a>]) -> Comment<'a> {
        let c: Vec<Comment<'a>> = docs.iter().map(Comment::from_app_info).collect();
        Comment::from_comment_vec(&c)
    }

    fn from_documentation_vec(docs: &[Documentation<'a>]) -> Comment<'a> {
        let c: Vec<Comment<'a>> = docs.iter().map(Comment::from_documentation).collect();
        Comment::from_comment_vec(&c)
    }

    fn from_comment_vec(c: &[Comment<'a>]) -> Comment<'a> {
        let text: Vec<&'a str> = c.iter().flat_map(|x| x.text.clone()).collect();
        Comment { text }
    }
}

#[cfg(test)]
mod test {
    use crate::abstract_code_model::comment::Comment;
    use crate::xsd_model::elements::annotation::Annotation;
    use crate::xsd_model::elements::notation::Notation;
    use crate::xsd_model::simple_types::id::Id;
    use crate::xsd_model::{AppInfo, Documentation};

    #[test]
    fn test_from_documentation() {
        let doc = Documentation {
            text: Some("A string"),
            ..Default::default()
        };
        let res = Comment::from_documentation(&doc);
        assert_eq!(res.text.len(), 1);
        assert_eq!(res.text[0], "A string");
    }

    #[test]
    fn test_from_app_info() {
        let app_info = AppInfo {
            text: Some("A string"),
            ..Default::default()
        };
        let res = Comment::from_app_info(&app_info);
        assert_eq!(res.text.len(), 1);
        assert_eq!(res.text[0], "A string");
    }

    #[test]
    fn test_from_annotation() {
        let annotation = Annotation {
            app_infos: vec![
                AppInfo {
                    text: Some("A string"),
                    ..Default::default()
                },
                Default::default(),
                Default::default(),
            ],
            documentations: vec![
                Documentation {
                    text: Some("Another string"),
                    ..Default::default()
                },
                Default::default(),
                Documentation {
                    text: Some("Yet another string"),
                    ..Default::default()
                },
            ],
            id: Some(Id("An id")),
            ..Default::default()
        };
        let res = Comment::from_annotation(&annotation);
        assert_eq!(res.text.len(), 3);
        assert_eq!(res.text[0], "A string");
        assert_eq!(res.text[1], "Another string");
        assert_eq!(res.text[2], "Yet another string");
    }

    #[test]
    fn test_from_notation() {
        let annotation = Annotation {
            app_infos: vec![AppInfo {
                text: Some("A string"),
                ..Default::default()
            }],
            ..Default::default()
        };

        let notation = Notation {
            annotation: Some(annotation),
            ..Default::default()
        };

        let res = Comment::from_notation(&notation);
        assert_eq!(res.text.len(), 1);
        assert_eq!(res.text[0], "A string");
    }
}
