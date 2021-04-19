use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, Span};
use syn::{Item, Ident,  parse_macro_input};
use quote::{quote, format_ident};

fn mk_error(span: Span, message: &str) -> TokenStream2{
  syn::Error::new(span,message).into_compile_error()
}

struct BitField{
  name: Ident,
  bit_type: Ident
}


pub fn expand(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as Item);
  let visibility: syn::Visibility;
  let ident: syn::Ident;
  let mut bit_fields = Vec::new();

  match input {
      syn::Item::Struct(item_struct) => {
          visibility = item_struct.vis;
          ident = item_struct.ident;
          item_struct.fields.iter().for_each(|field|{
              if let syn::Type::Path(syn::TypePath{ref path,..}) = &field.ty{
                  let bit_type = path.get_ident().unwrap().clone();
                  let name = field.ident.as_ref().unwrap().clone();
                  bit_fields.push(BitField{
                      name,
                      bit_type
                  });
              }
          })
      }
      _ => {
          return mk_error(Span::call_site(), "#[bitfield] can apply only to the struct.").into();
      }
  }

  let mut setters= Vec::new();
  let mut getters = Vec::new();

  bit_fields.iter().enumerate().for_each(|(i, bit_field)|{
      let name = &bit_field.name;
      let bit_type = &bit_field.bit_type;

      let bit_size = quote!{<#bit_type as Specifier>::BITS};
      let mut start_size = quote!{0};
      for k in 0..i{
          let prev_bit_type = &bit_fields[k].bit_type;
          start_size = quote!{#start_size + <#prev_bit_type as Specifier>::BITS};
      }
      let end_size = quote!{#start_size + #bit_size};
      let field_unit = quote!{<#bit_type as Specifier>::UNIT};
      let setter_ident = format_ident!("set_{}", name);
      let getter_ident = format_ident!("get_{}", name);

      let setter = quote!{
          fn #setter_ident(&mut self, value: #field_unit){
              let start_byte = (#start_size) / 8;
              let start_bit = (#start_size) % 8;
              let mut end_byte = (#end_size) / 8;
              let mut end_bit = (#end_size) % 8;
              let size = #bit_size;
              if end_bit == 0 {
                  end_byte -= 1;
                  end_bit = 8;
              }
              //clear existing data.

              self.data[start_byte] = self.data[start_byte].checked_shl((8 - start_bit) as u32).unwrap_or(0);
              self.data[start_byte] = self.data[start_byte].checked_shr((8 - start_bit) as u32).unwrap_or(0);
              self.data[end_byte] = self.data[end_byte].checked_shr(end_bit as u32).unwrap_or(0);
              self.data[end_byte] = self.data[end_byte].checked_shl(end_bit as u32).unwrap_or(0);

              if end_byte >  start_byte {
                  for i in (start_byte + 1)..=end_byte{
                      self.data[i] = 0;
                  }
              }

              let mut value_start_byte = value.checked_shl(start_bit as u32).unwrap_or(0) as u8;
              if start_byte == end_byte {
                  value_start_byte = value_start_byte.checked_shr((8 - end_bit) as u32).unwrap_or(0);
                  self.data[start_byte] = self.data[start_byte] | value_start_byte;
              }else{
                  let value_end_byte = value.checked_shr((size - end_bit) as u32).unwrap_or(0) as u8;
                  self.data[start_byte] = self.data[start_byte] | value_start_byte;
                  self.data[end_byte] = self.data[end_byte] | value_end_byte;
                  for i in (start_byte + 1)..end_byte{
                      let value_i_byte = value.checked_shr((start_bit + 8 * (i - start_byte -1)) as u32).unwrap_or(0) as u8;
                      self.data[i] = self.data[i] | value_i_byte;
                  }
              }       
          }
      };

      let getter = quote!{
          fn #getter_ident(&mut self) -> #field_unit{
              let start_byte = (#start_size) / 8;
              let start_bit = (#start_size) % 8;
              let mut end_byte = (#end_size) / 8;
              let mut end_bit = (#end_size) % 8;
              if end_bit == 0 {
                  end_byte -= 1;
                  end_bit = 8;
              }
              let mut value = self.data[start_byte] as #field_unit;

              if start_byte == end_byte{
                  value = value.checked_shl((8 - end_bit) as u32).unwrap_or(0);
                  value = value.checked_shr((8 - end_bit + start_bit) as u32).unwrap_or(0);
              }else{
                  value = value.checked_shr(start_bit as u32).unwrap_or(0);
                  for i in (start_byte + 1)..end_byte {
                      value += (self.data[i] as #field_unit).checked_shl((start_bit + 8 * (i - start_byte - 1)) as u32).unwrap_or(0);
                      
                  }
                  let mut byte = self.data[end_byte];

                  byte = byte.checked_shl((8 - end_bit) as u32).unwrap_or(0);
                  byte = byte.checked_shr((8 - end_bit) as u32).unwrap_or(0);
                  let mut byte = byte as #field_unit;
                  byte = byte.checked_shl((start_bit + 8 * (end_byte - start_byte - 1)) as u32).unwrap_or(0);
                  value += byte; 
              }
              value as #field_unit
          }
      };

      setters.push(setter);
      getters.push(getter);
  });

  let bit_types = bit_fields.iter().map(|bit_field|{
      let bit_type = &bit_field.bit_type;
      quote!{<#bit_type as Specifier>::BITS}
  });
  let total_bits = quote!{0 #(+ #bit_types)*};

  (quote!{
      #[repr(C)]
      #visibility struct #ident {
          data: [u8; (#total_bits) / 8],
      }

      impl ::bitfield::check::CheckTotalSizeMultipleOf8 for #ident{
          type Size = ::bitfield::check::TotalSize<[();  (#total_bits) % 8]>;
      }

      impl #ident {
          fn new() -> #ident{
              #ident{data: [0u8; (#total_bits) / 8]}
          }

          #(#setters)*
          #(#getters)*
      }
  }).into()
}