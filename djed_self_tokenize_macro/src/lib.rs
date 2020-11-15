
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

mod util;

#[proc_macro_derive(SelfTokenize)]
pub fn self_tokenize(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();
    let expanded = util::expand_macro(&ast, true, true);
    expanded.parse().unwrap()
}

#[proc_macro_derive(DefaultQuote)]
pub fn default_quote(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let source = input.to_string();
    let ast = syn::parse_derive_input(&source).unwrap();
    let expanded = util::expand_macro(&ast, true, false);
    expanded.parse().unwrap()
}
