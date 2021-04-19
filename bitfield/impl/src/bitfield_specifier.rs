use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, Span};
use syn::{DeriveInput, parse_macro_input};
use quote::quote;

pub fn derive(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let ident = &input.ident;

  //let mut field_values = Vec::new();

  let mut bits = 0;
  let mut unit_type = quote!{};
  if let syn::Data::Enum(syn::DataEnum{ref variants,..}) = input.data{
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
      const BITS : usize = #bits;
    }
  };

  TokenStream::from(expanded)
}