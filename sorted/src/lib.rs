use proc_macro::TokenStream;
use syn::{Item, parse_macro_input};

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let input = parse_macro_input!(input as Item);
    dbg!(input);

    TokenStream::new()
}
