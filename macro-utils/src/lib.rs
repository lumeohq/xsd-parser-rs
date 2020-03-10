extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

mod tuple;
mod union;

#[proc_macro_derive(UtilsTupleSerDe)]
pub fn tuple_serde(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let from_str = tuple::from_str(&ast);
    let display = tuple::display(&ast);
    let serde = tuple::serde(&ast);

    let ts = quote! {
        #from_str
        #display
        #serde
    };

    ts.into()
}

#[proc_macro_derive(UtilsUnionSerDe)]
pub fn union_serde(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    let serde = union::serde(&ast);

    let ts = quote! {
        #serde
    };

    ts.into()
}
