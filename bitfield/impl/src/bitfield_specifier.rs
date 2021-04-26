use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
use quote::quote;

pub fn derive(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let ident = &input.ident;

  let mut bits = 0;
  let mut unit_type = quote!{};
  let mut from_bytes_arms = Vec::new();
  if let syn::Data::Enum(syn::DataEnum{ref variants,..}) = input.data{
    from_bytes_arms = variants.iter().map(|variant|{
      let ident = &variant.ident;
      quote!{
        __field if __field == Self::#ident as <Self as Specifier>::UNIT => {
          Self::#ident
        }
      }
    }).collect();
    let count_variants = variants.iter().count();
    bits = count_variants.next_power_of_two().trailing_zeros() as usize;
    unit_type = match bits{
      0..=8 => quote!{u8},
      9..=16 => quote!{u16},
      17..=32 => quote!{u32},
      33..= 64 => quote!{u64},
      _ => unreachable!() 
    };
  }

  let expanded = quote!{
    impl Specifier for #ident{
      type UNIT = #unit_type;
      type InOut = #ident;
      const BITS : usize = #bits;

      fn to_bytes(input: Self::InOut) -> Self::UNIT{
        input as Self::UNIT
      }

      fn from_bytes(bytes: Self::UNIT) -> Self::InOut{
        match bytes {
          #(#from_bytes_arms),*
          _ => unreachable!()
        } 
      }
    }
  };

  TokenStream::from(expanded)
}