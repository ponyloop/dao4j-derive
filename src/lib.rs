#![feature(proc_macro_internals)]
#![feature(concat_idents)]


extern crate syn;
extern crate proc_macro;
extern crate proc_macro2;

use proc_macro2::{ Span };
use syn::token::{ Dot };
use syn::{ Data, Fields, Field, Ident };

use quote::{ quote };
use syn::punctuated::Pair;


#[proc_macro_derive(Label)]
pub fn label_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_label_macro(&ast)
}

fn impl_label_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    let name_as_string = &ast.ident.to_string();

    let fields: Option<&Fields> = match data {
        Data::Struct(a) => {
            Some(&a.fields)
        },
        _ => None
    };

    let named = match fields {
        Some(Fields::Named(fs)) => Some(fs.named.pairs()),
        _ => None
    };

    let fields: Vec<Ident> = named.unwrap()
        .map(|p| {
            p.into_value()
                .clone()
                .ident
                .unwrap()
        })
        .collect();

    let keys: Vec<String> = fields.clone().into_iter().map(|f| { f.to_string() }).collect();
    let values: Vec<Ident> = fields.clone();

    let output: proc_macro::TokenStream = quote! {
        impl Label for #name {
            fn to_string(&self) -> String {
                let mut result = Vec::new();
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


#[cfg(test)]
mod tests {

}
