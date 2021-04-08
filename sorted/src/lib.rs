use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, Span};
use syn::{Item, ItemEnum, ItemFn, ExprMatch, parse_macro_input};
use syn::visit_mut::{VisitMut};
use quote::quote;

fn mk_error(span: Span, message: &str) -> TokenStream2{
    syn::Error::new(span,message).into_compile_error()
}

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = TokenStream2::from(args);
    let item = input.clone();
    let item = parse_macro_input!(item as Item);
    if let Item::Enum(ItemEnum{ref variants,..}) = item{
        let variants = variants.clone().into_pairs().map(|pair|{
            pair.value().ident.clone()
        }).collect::<Vec<_>>();
        for i in 1..variants.len(){
            let ident = variants.get(i).unwrap();
            for j in 0..i{
                let prev_ident = variants.get(j).unwrap();
                if prev_ident.to_string() > ident.to_string(){
                    let message = format!("{} should sort before {}",ident.to_string(),prev_ident.to_string());
                    let error = mk_error(ident.span(), &message);
                    return (quote!{#error}).into();
                }
            }
        }
    }else{
        let error =  mk_error(Span::call_site(),"expected enum or match expression");
        return (quote!{#error}).into();
    }

    input
}

struct SortMatchArm{
    error_token: Option<TokenStream2>
}

impl VisitMut for SortMatchArm{
    fn visit_expr_match_mut(&mut self, node: &mut ExprMatch) {
        let node_copy = node.clone();
        let attr = node_copy.attrs.iter().enumerate().find_map(|(i,attr)|{
            if attr.path.is_ident("sorted"){
                node.attrs.remove(i);
                Some(attr)
            }else{
                None
            }
        });

        if attr.is_some(){
            let arms = node_copy.arms.iter().filter_map(|arm|{
                if let syn::Pat::TupleStruct(syn::PatTupleStruct{ref path,..}) = arm.pat{
                    path.get_ident()
                }else{
                    None
                }
            }).collect::<Vec<_>>();
            
            for i in 1..arms.len(){
                let ident = *arms.get(i).unwrap();
                for j in 0..i{
                    let prev_ident = *arms.get(j).unwrap();
                    if prev_ident.to_string() > ident.to_string(){
                        let message = format!("{} should sort before {}",ident.to_string(),prev_ident.to_string());
                        self.error_token = Some(mk_error(ident.span(), &message));
                    }
                }
            }
        }
    }
}

#[proc_macro_attribute]
pub fn check(_: TokenStream, input: TokenStream) -> TokenStream{
    let mut input = parse_macro_input!(input as ItemFn);    
    let mut sort_match_arm = SortMatchArm{error_token:None};
    sort_match_arm.visit_item_fn_mut(&mut input);
    if let Some(error_token) = sort_match_arm.error_token{
        return (quote! { #input #error_token }).into();
    }
    (quote!{#input}).into()
}
