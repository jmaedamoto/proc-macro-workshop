mod expand;
mod bitfield_specifier;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn bitfield(_: TokenStream, input: TokenStream) -> TokenStream {
    expand::expand(input)
}

#[proc_macro_derive(BitfieldSpecifier)]
pub fn derive(input: TokenStream) -> TokenStream {
    bitfield_specifier::derive(input)
}
