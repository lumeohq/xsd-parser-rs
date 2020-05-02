use crate::abstract_code_model::app_info::AppInfo;
use crate::abstract_code_model::comment::Comment;
use crate::abstract_code_model::id::Id;

#[derive(Debug, Default)]
pub struct Annotation<'a> {
    pub app_infos: Vec<AppInfo<'a>>,
    pub comment: Comment<'a>,
    pub id: Id<'a>,
    // TODO: Vendor specific attributes support.
}
