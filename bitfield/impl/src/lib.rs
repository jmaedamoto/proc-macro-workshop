mod expand;
mod bitfield_specifier;
mod define_default_type;

use proc_macro::TokenStream;

#[proc_macro]
pub fn define_default_type(_: TokenStream) -> TokenStream{
    TokenStream::from(define_default_type::define_default_type())
}

#[proc_macro_attribute]
pub fn bitfield(_: TokenStream, input: TokenStream) -> TokenStream {
    expand::expand(input)
}

#[proc_macro_derive(BitfieldSpecifier)]
pub fn derive(input: TokenStream) -> TokenStream {
    bitfield_specifier::derive(input)
}
