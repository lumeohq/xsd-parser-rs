use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

enum Type<'a> {
    Simple(&'a syn::Path),
    String(&'a syn::Path),
    Struct(&'a syn::Path),
    Vec(&'a syn::Path, &'a syn::Path),
}

pub fn serde(ast: &syn::DeriveInput) -> syn::Result<TokenStream> {
    let from_str = from_str(&ast)?;
    let display = display(&ast)?;

    Ok(quote! {
        #from_str
        #display
    })
}

fn from_str(ast: &syn::DeriveInput) -> syn::Result<TokenStream> {
    let convert = match extract_field_type(ast) {
        Type::String(_) => quote! { s.to_string() },
        Type::Struct(ty) | Type::Simple(ty) => {
            quote! { <#ty as ::std::str::FromStr>::from_str(s).map_err(|e| e.to_string())? }
        }
        Type::Vec(_, subtype) => match Type::from_path(&subtype) {
            Type::String(subtype) | Type::Struct(subtype) | Type::Simple(subtype) => quote! {
                s.split_whitespace()
                    .filter_map(|s| <#subtype as ::std::str::FromStr>::from_str(s).ok())
                    .collect()
            },
            _ => {
                return Err(syn::Error::new(
                    subtype.span(),
                    "Not implemented for this subtype",
                ))
            }
        },
    };

    let struct_name = &ast.ident;

    Ok(quote! {
        impl ::std::str::FromStr for #struct_name {
            type Err = ::std::string::String;

            fn from_str(s: &::std::primitive::str) -> ::std::result::Result<Self, Self::Err> {
                Ok(#struct_name(#convert))
            }
        }
    })
}

fn display(ast: &syn::DeriveInput) -> syn::Result<TokenStream> {
    let write = match extract_field_type(ast) {
        Type::String(_) | Type::Simple(_) | Type::Struct(_) => quote! {
            write!(f, "{}", self.0)
        },
        Type::Vec(_, subtype) => match Type::from_path(&subtype) {
            Type::String(_) | Type::Simple(_) | Type::Struct(_) => quote! {
                let mut it = self.0.iter();
                if let Some(val) = it.next() {
                    write!(f, "{}", val)?;
                }
                for val in it {
                    write!(f, " {}", val)?;
                }

                Ok(())
            },
            _ => {
                return Err(syn::Error::new(
                    subtype.span(),
                    "Not implemented for this subtype",
                ))
            }
        },
    };

    let struct_name = &ast.ident;

    Ok(quote! {
        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #write
            }
        }
    })
}

impl Type<'_> {
    pub fn from_path(path: &syn::Path) -> Type {
        match path
            .segments
            .last()
            .expect("Empty type")
            .ident
            .to_string()
            .as_str()
        {
            "bool" | "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" | "f32"
            | "f64" => Type::Simple(path),
            "String" => Type::String(path),
            "Vec" => Type::Vec(
                path,
                extract_subtype(path.segments.last().expect("Missing subtype"))
                    .expect("Vec subtype not found"),
            ),
            _ => Type::Struct(path),
        }
    }
}

fn extract_field_type(ast: &syn::DeriveInput) -> Type {
    match &ast.data {
        syn::Data::Struct(data_struct) => {
            let field_path = extract_field_path(&data_struct).expect("Bad field count or type");

            Type::from_path(&field_path)
        }
        _ => unimplemented!("Implemented only for structs"),
    }
}

fn extract_field_path(data_struct: &syn::DataStruct) -> Option<&syn::Path> {
    if let syn::Fields::Unnamed(fields) = &data_struct.fields {
        if let Some(field) = fields.unnamed.first() {
            if let syn::Type::Path(path) = &field.ty {
                return Some(&path.path);
            }
        }
    }

    None
}

fn extract_subtype(path: &syn::PathSegment) -> Option<&syn::Path> {
    if let syn::PathArguments::AngleBracketed(args) = &path.arguments {
        if let Some(syn::GenericArgument::Type(syn::Type::Path(path))) = args.args.last() {
            return Some(&path.path);
        }
    }

    None
}
