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

#[proc_macro_attribute]
pub fn bitfield(_: TokenStream, input: TokenStream) -> TokenStream {

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

    let bit_types = bit_fields.iter().map(|bit_field|{
        let bit_type = &bit_field.bit_type;
        quote!{<#bit_type as Specifier>::BITS}
    });

    let size = quote!{(0 #(+ #bit_types)*) / 8};

    //let mut setters = Vec::new();
    //let mut getters = Vec::new();

    bit_fields.iter().enumerate().for_each(|(i,bit_field)|{
        
        let setter = quote!{
            fn set_c(&mut self, c: u64){
                let byte = (<B1 as Specifier>::BITS + <B3 as Specifier>::BITS) / 8;
                let bit = <B1 as Specifier>::BITS + <B3 as Specifier>::BITS;
        
                let mut mask:[u8;4] = [255,255,255,255];
                mask[byte] = mask[byte].checked_shl(bit as u32).unwrap() as u8;
        
                let mut value:[u8;4] = [0,0,0,0];
                value[byte] = c.checked_shl(bit as u32).unwrap() as u8;
        
                for i in 0..self.data.len(){
                    self.data[i] = self.data[i] & mask[i] | value[i];
                }
            }
        };
    });

    (quote!{
        #[repr(C)]
        #visibility struct #ident {
            data: [u8; #size],
        }

        impl #ident {
            fn new() -> #ident{
                #ident{data: [0u8; #size]}
            }
        }
    }).into()
}
