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
