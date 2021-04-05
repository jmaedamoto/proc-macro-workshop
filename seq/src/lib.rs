use proc_macro::TokenStream;
use syn::{parse_macro_input,Token,braced};
use syn::parse::{Parse,ParseStream,Result};

struct Item {
    ident: syn::Ident,
    start_num: syn::LitInt,
    end_num: syn::LitInt,
}

impl Parse for Item{
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: syn::Ident = input.parse()?;
        input.parse::<Token![in]>()?;
        let start_num: syn::LitInt = input.parse()?;
        input.parse::<Token![..]>()?;
        let end_num: syn::LitInt = input.parse()?;
        let content;
        let braced_token = braced!(content in input);
        Ok( Item{
            ident,
            start_num,
            end_num,
        })
    }
}

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);

    TokenStream::new()
}
