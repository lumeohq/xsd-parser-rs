use crate::abstract_code_model::annotation::Annotation;
use crate::abstract_code_model::app_info::AppInfo;
use crate::abstract_code_model::comment::Comment;
use crate::abstract_code_model::id::Id;
use crate::xsd_model::elements::annotation::Annotation as XsdAnnotation;

impl<'a> Annotation<'a> {
    pub fn parse(annotation: &XsdAnnotation<'a>) -> Annotation<'a> {
        Annotation {
            app_infos: annotation
                .app_infos
                .iter()
                .map(|x| AppInfo::parse(x))
                .collect(),
            comment: Comment::parse_vec(&annotation.documentations),
            id: Id::parse(&annotation.id),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::abstract_code_model::annotation::Annotation;
    use crate::xsd_model::elements::annotation::Annotation as XsdAnnotation;
    use crate::xsd_model::elements::documentation::Documentation;
    use crate::xsd_model::simple_types::id::Id as XsdId;

    #[test]
    fn test_annotation_parse() {
        let annotation = XsdAnnotation {
            app_infos: vec![Default::default(), Default::default(), Default::default()],
            documentations: vec![
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
            ],
            id: Some(XsdId("An id")),
            ..Default::default()
        };
        let res = Annotation::parse(&annotation);
        assert_eq!(res.app_infos.len(), 3);
        assert_eq!(res.comment.text.len(), 2);
        assert_eq!(res.id.0, Some("An id"));
    }
}
