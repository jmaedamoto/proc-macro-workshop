use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, Span};
use syn::{Item, parse_macro_input};
use quote::quote;

fn mk_error(span: Span, message: &str) -> TokenStream2{
    syn::Error::new(span,message).into_compile_error()
}

#[proc_macro_attribute]
pub fn bitfield(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);
    let visibility: syn::Visibility;
    let ident: syn::Ident;
    let mut bit_types = Vec::new();

    match input {
        syn::Item::Struct(item_struct) => {
            visibility = item_struct.vis;
            ident = item_struct.ident;
            item_struct.fields.iter().for_each(|field|{
                if let syn::Type::Path(syn::TypePath{ref path,..}) = field.ty{
                    let bit = path.get_ident().unwrap();
                    bit_types.push(quote!{ + <#bit as Specifier>::BITS});
                }
            })
        }
        _ => {
            return mk_error(Span::call_site(), "#[bitfield] can apply only to the struct.").into();
        }
    }

    let size = quote!{(0 #(#bit_types)*) / 8};

    (quote!{
        #[repr(C)]
        #visibility struct #ident {
            data: [u8; #size],
        }
    }).into()
}
