use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_quote, punctuated::Punctuated, Data, DataStruct, DeriveInput, FieldValue, Fields, Token,
};

pub fn expand_round2(input: DeriveInput) -> syn::Result<TokenStream> {
    let round2: syn::Path = parse_quote!(howitt::services::num::Round2);

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    let field_values = fields
        .into_iter()
        .map(|f| {
            let field_name = f.ident;

            Ok(parse_quote! {
                #field_name: #round2::round2(self.#field_name)
            })
        })
        .collect::<syn::Result<Vec<FieldValue>>>()?;

    let field_values =
        Punctuated::<FieldValue, Token![,]>::from_iter(field_values).into_token_stream();

    let st_name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics #round2 for #st_name #ty_generics #where_clause {
            // #getters
            fn round2(self) -> Self {
                #st_name {
                    #field_values
                }
            }
        }
    })
}
