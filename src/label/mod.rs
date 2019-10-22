use proc_macro2::{ Span };
use syn::token::{ Dot, Comma };
use syn::{ Data, Fields, Field, Ident };

use quote::{ quote };
use syn::punctuated::{ Pair, IntoPairs };

use dao4j::Label;

fn get_fields(data: &Data) -> Option<&Fields> {
    match data {
        Data::Struct(a) => {
            Some(&a.fields)
        },
        _ => None
    }
}

fn get_named_fields(fields: Option<&Fields>) -> Option<IntoPairs<Field, Comma>> {
    match fields {
        Some(Fields::Named(fs)) => Some(fs.named.clone().into_pairs()),
        _ => None
    }
}

pub fn impl_label_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    let name_as_string = &ast.ident.to_string();

    let struct_fields: Option<&Fields> = get_fields(data);
    let named = get_named_fields(struct_fields);

    let fields: Vec<Option<Ident>> = named.unwrap()
        .map(|p| { p.into_value().ident })
        .filter(|i| { i.is_some() })
        .collect();

    let keys: Vec<String> = fields
        .clone()
        .into_iter()
        .map(|f| { f.unwrap().to_string() })
        .collect();
    let keys_len = keys.len();
    let values: Vec<Option<Ident>> = fields.clone();

    let output: proc_macro::TokenStream = quote! {
        impl Label for #name {
            fn to_query_string(&self) -> String {
                let mut result = Vec::with_capacity(#keys_len);
                #( result.push(format!("{}: {}", &#keys, self.#values)); )*

                format!(
                    "{} {{ {} }}",
                    &#name_as_string,
                    &result.as_slice().join(",")
                )
            }
        }
    }.into();

    proc_macro::TokenStream::from(output)
}
