use crate::abstract_code_model::annotation::Annotation;
use crate::abstract_code_model::any_uri::AnyUri;
use crate::abstract_code_model::id::Id;
use crate::abstract_code_model::ncname::NCName;
use crate::abstract_code_model::public::Public;

#[derive(Debug, Default)]
pub struct Notation<'a> {
    pub annotation: Option<Annotation<'a>>,
    pub id: Id<'a>,
    pub name: NCName<'a>,
    pub public: Option<Public<'a>>,
    pub system: Option<AnyUri<'a>>,
    // TODO: Vendor specific attributes support.
}
