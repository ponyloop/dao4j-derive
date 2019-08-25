#![feature(proc_macro_internals)]
#![feature(concat_idents)]


extern crate syn;
extern crate proc_macro;
extern crate proc_macro2;


mod label;

use label::{ impl_label_macro };

#[proc_macro_derive(Label)]
pub fn label_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_label_macro(&ast)
}


#[cfg(test)]
mod tests {

}