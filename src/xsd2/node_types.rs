use crate::xsd2::node_traits::{
    AnyElement as AnyElementTrait,
    Attributes,
    Choice as ChoiceTrait,
    Documentation,
    Elements,
    Extension as ExtensionTrait,
    MinMaxOccurs,
    Name,
    Node,
    Sequence as SequenceTrait,
    TypeName,
};
use crate::xsd2::utils::find_child;

macro_rules! create_node_type{
    ($name:ident) => {
        pub struct $name<'a, 'input> {
            pub node: roxmltree::Node<'a, 'input>,
        }

        impl Node for $name<'_, '_>{
           fn node(&self) -> &roxmltree::Node {
                &self.node
           }
        }
    };
    ($name:ident, $($ty:ty), *) => {
        create_node_type!{$name}
        $(impl $ty for $name<'_, '_>{})*
    };
    ($name:ident, $($ty:ty),+ ,) => {
        create_node_type!($name, $($ty), *);
    };
}
create_node_type!(AnyElement);

create_node_type! (Choice,
    Elements,
    MinMaxOccurs,
);

create_node_type!(Element,
    Documentation,
    MinMaxOccurs,
    Name,
    TypeName,
);

create_node_type!(Sequence,
    AnyElementTrait,
    ChoiceTrait,
    Elements,
);

create_node_type!(Extension, SequenceTrait, Attributes);

impl<'a, 'input: 'a> Extension<'a, 'input> {
    pub fn has_any_attribute(&self) -> bool {
        find_child(&self.node, "anyAttribute").is_some()
    }

    pub fn base(&self) -> &'a str {
        self.node.attribute("base").expect("base required attribute for extension")
    }
}

create_node_type!(SimpleContent, ExtensionTrait);

create_node_type!(Attribute,
    Name,
    Documentation,
    TypeName,
);

pub enum UseType {
    Optional,
    Prohibited,
    Required,
}

impl<'a, 'input> Attribute<'a, 'input> {
    pub fn use_type(&self) -> UseType {
        match self.node.attribute("use") {
            Some(a) => match a {
                "optional" => UseType::Optional,
                "prohibited" => UseType::Prohibited,
                "required" => UseType::Required,
                _ => UseType::Optional
            },
            None => UseType::Optional
        }
    }
}
