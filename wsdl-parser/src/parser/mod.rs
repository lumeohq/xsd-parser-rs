use crate::parser::types::Definitions;

pub mod types;
mod constants;

pub fn parse(text: &str)  {
    let doc = roxmltree::Document::parse(&text).expect("Parse document error");
    let root = doc.root();
    println!("{}", root.children().count());



    let definitions = root
        .children()
        .filter(|e| e.is_element())
        .last()
        .expect("Definitions element is required");

    // for ch in definitions.children() {
    //     println!("{:?}", ch);
    // }

    let fc = definitions.first_child().unwrap();

    println!("{:?}", fc);
    println!("{:?}", fc.next_sibling_element().unwrap());
    println!("{:?}", fc.next_sibling_element().unwrap());

    let def = Definitions::new(&definitions);
    println!("{:?}", def.imports().len());
    println!("{:?}", def.schemas().len());
    println!("{:?}", def.messages().len());

    // for i in def.imports() {
    //     println!("{:#?}", i);
    // }
}



