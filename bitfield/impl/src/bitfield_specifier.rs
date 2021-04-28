use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{DeriveInput, parse_macro_input};
use quote::{quote, quote_spanned};

pub fn derive(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let enum_ident = &input.ident;

  let mut bits = 0;
  let mut unit_type = quote!{};
  let mut from_bytes_arms = Vec::new();
  let mut discriminant_in_range_check = Vec::new();
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
    if !count_variants.is_power_of_two(){
      let message = "BitfieldSpecifier expected a number of variants which is a power of 2";
      let error_token = syn::Error::new(Span::call_site(),message).into_compile_error();
      return TokenStream::from(error_token);
    }

    bits = count_variants.next_power_of_two().trailing_zeros() as usize;
    unit_type = match bits{
      0..=8 => quote!{u8},
      9..=16 => quote!{u16},
      17..=32 => quote!{u32},
      33..= 64 => quote!{u64},
      _ => unreachable!() 
    };

    discriminant_in_range_check = variants.iter().map(|variant|{
      let ident = &variant.ident;
      let span = ident.span();
      quote_spanned!(span => 
        impl ::bitfield::check::CheckDiscriminantInRange<[(); Self::#ident as usize]> for #enum_ident{
          type CheckType = [(); ((Self::#ident as usize) < (0x01_usize << #bits)) as usize];
        }
      )
    }).collect();
  }

  let expanded = quote!{
    #(#discriminant_in_range_check)*

    impl Specifier for #enum_ident{
      type UNIT = #unit_type;
      type InOut = #enum_ident;
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