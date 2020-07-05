use crate::abstract_code_model::TypeName;
use crate::xsd_model::simple_types::qname::QName;
use std::borrow::Cow;

impl<'a> TypeName<'a> {
    pub fn from_qname(qname: &'a QName) -> Self {
        Self {
            namespace: qname.prefix.map(|x| Cow::from(x)),
            name: qname.name.into()
        }
    }
}