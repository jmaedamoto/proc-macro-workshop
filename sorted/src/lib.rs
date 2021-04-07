use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, Span};
use syn::{Item, parse_macro_input};

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = TokenStream2::from(args);
    let input = parse_macro_input!(input as Item);
    if let Item::Enum(_) = input{

    }else{
        return TokenStream::from( syn::Error::new(Span::call_site(),"expected enum or match expression").into_compile_error());
    }

    TokenStream::new()
}
