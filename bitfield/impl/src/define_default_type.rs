use proc_macro2::TokenStream;
use quote::{quote, format_ident};

pub fn define_default_type() -> TokenStream{
  let bit_types = (1..=64).map(generate_bit_type);
  quote!{ #(#bit_types)*}
}

fn generate_bit_type(bits: usize) -> TokenStream{
  let unit_type = match bits{
    1..=8 => quote!{::core::primitive::u8},
    9..=16 => quote!{::core::primitive::u16},
    17..=32 => quote!{::core::primitive::u32},
    18..=64 => quote!{::core::primitive::u64},
    _ => unreachable!(),
  };

  let ident = format_ident!("B{}", bits);

  quote!{
    pub enum #ident {}
    impl Specifier for #ident{
      type UNIT = #unit_type;
      type InOut = #unit_type;
      const BITS : usize = #bits;

      fn from_bytes(bytes: #unit_type) -> #unit_type {
        bytes
      }
    
      fn to_bytes(input: #unit_type) -> #unit_type {
        input
      }
    }
  }
}