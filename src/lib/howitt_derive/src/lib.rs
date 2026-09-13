use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod round2;
use round2::expand_round2;

#[proc_macro_derive(Round2, attributes(getter))]
pub fn round2(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_round2(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
