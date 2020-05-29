extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn datatype(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = attr.to_string();
    let item2 = item.clone();
    let inp = syn::parse_macro_input!(item as syn::Item);
    let item = item2.to_string();
    let types = attr.split(" ");
    let code: proc_macro2::TokenStream = types
        .fold("".to_string(), |mut string, datatype| {
            string.push_str(&item.clone().replace("f32", datatype));
            string
        })
        .parse()
        .unwrap();
    let code = quote! {
        #inp
        #code
    };
    TokenStream::from(code)
}
