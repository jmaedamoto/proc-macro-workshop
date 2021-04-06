use proc_macro::TokenStream;
use syn::{parse_macro_input, Ident, LitInt, Token, braced};
use syn::parse::{Parse};

struct Item {
    ident: Ident,
}

impl Parse for Item {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        input.parse::<Token![in]>()?;
        let _ : LitInt = input.parse()?;
        input.parse::<Token![..]>()?;
        let _ : LitInt = input.parse()?;
        let content;
        let _ = braced!(content in input);
        Ok(Item{
            ident
        })
    }
}



#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);

    TokenStream::new()
}
