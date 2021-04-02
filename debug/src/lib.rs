use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
use quote::quote;

#[proc_macro_derive(CustomDebug)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let ident_str = ident.to_string();
    let fields  = if let syn::Data::Struct(syn::DataStruct{ref fields,..}) = input.data {
        fields
    }else{
        return TokenStream::from(syn::Error::new(ident.span(),"Custom Debug corresponds to Struct only").to_compile_error());
    };

    let fields_token = fields.iter().map(|f|{
        match &f.ident{
            Some(name) => {
                let name_str = name.to_string();
                quote!{ .field(#name_str, &self.#name) }
            }
            None => quote!{}
        }
    });

    let expanded = quote!{
        impl ::std::fmt::Debug for #ident{
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct(#ident_str)
                 #(#fields_token)*
                 .finish()
            }
        }
    };

    TokenStream::from(expanded)
}
