use crate::xml_to_xsd::XsdNode;
use crate::xsd_model::elements::ElementType;
use crate::xsd_model::groups::simple_derivation::SimpleDerivation;
use crate::xsd_model::simple_types::ncname::NCName;
use crate::xsd_model::simple_types::simple_derivation_set::SimpleDerivationSet;
use crate::xsd_model::{Annotation, Restriction};
use crate::xsd_model::{List, LocalSimpleType, TopLevelSimpleType, Union};
use roxmltree::Node;

impl<'a> LocalSimpleType<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<LocalSimpleType<'a>, String> {
        let mut annotation = None;
        let mut id = None;
        let mut attributes = vec![];
        for attr in node.attributes() {
            match attr.name() {
                "id" => id = Some(attr.into()),
                _ => attributes.push(attr.clone()),
            }
        }

        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Annotation => annotation = Some(Annotation::parse(ch)?),
                x => {
                    return Ok(Self {
                        annotation,
                        content_choice: SimpleDerivation::parse(ch, x)?,
                        id,
                        attributes,
                    })
                }
            };
        }

        Err("".to_string())
    }
}

impl<'a> TopLevelSimpleType<'a> {
    pub fn parse(node: Node<'a, '_>) -> Result<Self, String> {
        let mut annotation = None;
        let mut content_choice = None;
        let mut id = None;
        let mut final_: Option<SimpleDerivationSet> = None;
        let mut name: Option<NCName> = None;
        let mut attributes = vec![];

        for ch in node.children().filter(|n| n.is_element()) {
            match ch.xsd_type()? {
                ElementType::Annotation => annotation = Some(Annotation::parse(ch)?),
                x => content_choice = Some(SimpleDerivation::parse(ch, x)?),
            };
        }

        let content_choice = content_choice.ok_or_else(|| {
            format!(
                "Content required for xsd:TopLevelSimpleType type: {:?}",
                node
            )
        })?;

        for attr in node.attributes() {
            match attr.name() {
                "id" => id = Some(attr.into()),
                "final" => final_ = Some(SimpleDerivationSet::parse(attr.value())?),
                "name" => name = Some(attr.into()),
                _ => attributes.push(attr.clone()),
            }
        }

        let name = name.ok_or_else(|| {
            format!(
                "Content required for xsd:TopLevelSimpleType type: {:?}",
                node
            )
        })?;

        Ok(Self {
            annotation,
            content_choice,
            id,
            final_,
            name,
            attributes,
        })
    }
}

impl<'a> SimpleDerivation<'a> {
    pub fn parse(
        node: Node<'a, '_>,
        element_type: ElementType,
    ) -> Result<SimpleDerivation<'a>, String> {
        let res = match element_type {
            ElementType::Union => Self::Union(Box::new(Union::parse(node)?)),
            ElementType::Restriction => Self::Restriction(Box::new(Restriction::parse(node)?)),
            ElementType::List => Self::List(Box::new(List::parse(node)?)),
            _ => return Err(format!("Invalid simple derivation content: {:?}", node)),
        };

        Ok(res)
    }
}

#[cfg(test)]
mod test {

    use crate::xsd_model::groups::simple_derivation::SimpleDerivation;
    use crate::xsd_model::TopLevelSimpleType;

    #[test]
    fn test_top_level_simple_type_parse() {
        let doc = roxmltree::Document::parse(
            r##"<simpleType id="ID" name="Type1" final="#all" a='b' b='a'>
                        <list itemType="itemType" />
                    </simpleType>"##,
        )
        .unwrap();
        let root = doc.root_element();
        let res = TopLevelSimpleType::parse(root).unwrap();
        assert!(res.annotation.is_none());
        assert_eq!(res.attributes.len(), 2);
        assert_eq!(res.id.unwrap().0, "ID");
        assert_eq!(res.name.0, "Type1");
        match &res.content_choice {
            SimpleDerivation::List(x) => assert_eq!(x.item_type.as_ref().unwrap().name, "itemType"),
            _ => unreachable!("test failed!"),
        }
    }
}
