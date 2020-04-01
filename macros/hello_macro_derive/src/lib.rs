extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // look into syn documentation
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! {}", stringify!(#name));
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // construct rust code as a syntax tree
    let ast = syn::parse(input).unwrap();

    // build the trait implementation
    impl_hello_macro(&ast)
}
