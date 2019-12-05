use crate::xsd2::simple_type::{SimpleType, List, FacetType, Facet, Restriction};
use crate::generator::utils::*;


fn tuple_struct(doc: &String, name: &String, typename: &String) -> String {
    format!("{} pub struct {} ({}); \n\n", doc, name, typename)
}

fn enum_struct(name: &String, cases: &String) -> String {
    format!("pub enum {} {{\n{}\n}}\n\n", name, cases)
}

fn list_simple_type(doc: &String, name: &String, list: &List) -> String {
    tuple_struct(
        doc,
        name,
        &format!("Vec<{}>", list.item_type().unwrap_or("UNSUPPORT NESTED LIST TYPE"))
    )
}


fn enum_case(facet : &Facet, typename: &str) -> String {
    format!("{}  {}({}),",
            get_struct_comment(facet.documentation()),
            get_type_name(facet.value()),
            typename
    )
}

fn enum_cases(facets: &Vec<Facet>, typename: &str) -> String {
    facets.
        iter().
        map(|f| enum_case(f, typename)).
        collect::<Vec<String>>().join("\n")
}


fn get_enum_facets<'a, 'input>(restriction: &Restriction<'a, 'input>) -> Vec<Facet<'a, 'input>> {
    restriction.facets().
    into_iter().
    filter(|f| match f.facet_type {
        FacetType::Enumeration => true,
        _ => false
    }).collect()
}

pub fn simple_type(element: &SimpleType) -> String {
    let doc = get_struct_comment(element.documentation());
    let name = get_type_name(element.name().expect("SIMPLE TYPE WITHOUT NAME NO SUPPORT"));

    let l = element.list();
    if l.is_some() {
        return list_simple_type(&doc, &name, &l.unwrap());
    }

    let restriction = element.restriction();
    if restriction.is_some() {
        let r = restriction.unwrap();
        let facets = get_enum_facets(&r);

        if !facets.is_empty() {
            return enum_struct(&name, &enum_cases(&facets,r.base()));
        }

        return tuple_struct(&doc, &name, &r.base().to_string());
    }

    return format!("{} = {:?}", "UNSUPPORTED SIMPLE TYPE", element);
}