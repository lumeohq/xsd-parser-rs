use crate::parser::types::Definitions;

pub mod binding;
mod constants;
pub mod port_type;
pub mod types;

pub fn parse(text: &str) {
    let doc = roxmltree::Document::parse(&text).expect("Parse document error");
    let root = doc.root();
    println!("{}", root.children().count());

    let definitions = root
        .children()
        .filter(|e| e.is_element())
        .last()
        .expect("Definitions element is required");

    let def = Definitions::new(&definitions);
    println!("{:?}", def.imports().len());
    println!("{:?}", def.schemas().len());
    println!("{:?}", def.messages().len());

    // for i in def.imports() {
    //     println!("{:#?}", i);
    // }
}

pub trait WsdlElement {
    fn wsdl_type(&self) -> ElementType;
}

#[derive(Debug, PartialEq)]
pub enum ElementType {
    Binding,
    Definitions,
    Documentation,
    Import,
    Input,
    Fault,
    Message,
    Operation,
    Output,
    Part,
    PortType,
    Types,
    UnknownElement(String),
}

impl<'a> WsdlElement for roxmltree::Node<'a, '_> {
    fn wsdl_type(&self) -> ElementType {
        use ElementType::*;
        // TODO: check for wsdl prefix
        match self.tag_name().name() {
            "binding" => Binding,
            "definitions" => Definitions,
            "documentation" => Documentation,
            "import" => Import,
            "input" => Input,
            "fault" => Fault,
            "message" => Message,
            "operation" => Operation,
            "output" => Output,
            "part" => Part,
            "portType" => PortType,
            "types" => Types,
            _ => UnknownElement(self.tag_name().name().to_string()),
        }
    }
}
