use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, Span};
use syn::{Item, ItemEnum, parse_macro_input};

fn mk_error(span: Span, message: &str) -> TokenStream{
    TokenStream::from(syn::Error::new(span,message).into_compile_error())
}

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = TokenStream2::from(args);
    let input = parse_macro_input!(input as Item);
    if let Item::Enum(ItemEnum{ref variants,..}) = input{
        let variants = variants.clone().into_pairs().map(|pair|{
            pair.value().ident.clone()
        }).collect::<Vec<_>>();
        for i in 1..variants.len(){
            let ident = variants.get(i).unwrap();
            for j in 0..(i - 1){
                let prev_ident = variants.get(j).unwrap();
                if prev_ident.to_string() > ident.to_string(){
                    let message = format!("{} should sort before {}",ident.to_string(),prev_ident.to_string());
                    return mk_error(ident.span(), &message);
                }
            }
        }
    }else{
        return mk_error(Span::call_site(),"expected enum or match expression");
    }

    TokenStream::new()
}
