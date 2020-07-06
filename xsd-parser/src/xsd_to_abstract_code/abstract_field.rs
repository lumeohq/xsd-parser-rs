use crate::abstract_code_model::abstract_field::AbstractField;
use crate::xsd_model::LocalElement;
use std::borrow::Cow;
use crate::abstract_code_model::TypeName;
use crate::abstract_code_model::comment::Comment;

// impl<'a> AbstractField<'a> {
//     pub fn from_element(elem: &'a LocalElement) -> Result<Self, String> {
//         if let Some(r) = elem.ref_.as_ref() {
//             Ok(
//                 Self {
//                     name: Some(Cow::from(r.name)),
//                     type_name: Some(TypeName::from_qname(r)),
//                     comment: Comment::from_opt_annotation(&elem.annotation),
//                     inner_type: None,  // If 'ref' is presented inner type not allowed
//                     extensions: Default::default(),
//                 }
//             )
//         } else if let Some(name) = elem.name.as_ref() {
//             //type or inner_type must be presented
//             Ok(
//                 Self {
//                     name: name.into(),
//                     type_name: elem.type_.as_ref().map(|ty| TypeName::from_qname(ty)),
//                     comment: Comment::from_opt_annotation(&elem.annotation),
//                     inner_type: None,
//                     extensions: Default::default(),
//                 }
//             )
//         } else {
//             Err("Either 'name' or 'ref' attribute should present".to_string())
//         }
//     }
// }