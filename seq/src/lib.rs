use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, Ident, LitInt, Token, braced};
use syn::parse::Parse;
#[derive(Debug)]
struct Item {
    ident: Ident,
    content: TokenStream2,
}

impl Parse for Item {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        input.parse::<Token![in]>()?;
        let _ : LitInt = input.parse()?;
        input.parse::<Token![..]>()?;
        let _ : LitInt = input.parse()?;
        let content: syn::parse::ParseBuffer;
        let _braces = braced!(content in input);
        let content: TokenStream2 = content.parse()?;
        Ok(Item{
            ident,
            content,
        })
    }
}



#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);
    TokenStream::new()
}
