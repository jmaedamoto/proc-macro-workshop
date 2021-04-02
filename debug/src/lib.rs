use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
use quote::quote;

fn extract_debug_pattern(field: &syn::Field) -> Option<String>{
    let attrs = &field.attrs;
    attrs.iter().find_map(|attr| {
        let meta = attr.parse_meta();
        match meta {
            Ok(syn::Meta::NameValue(ref meta_name_value)) if meta_name_value.path.is_ident("debug") => {
                if let syn::Lit::Str(ref lit_str) = meta_name_value.lit {
                    Some(lit_str.value())
                }else{
                    None
                }
            },
            _ => None
        }
    })
}

fn extract_generic_types(ty: &syn::Type, generic_types: &Vec::<syn::Ident>) -> Option<syn::Path>{
    if let syn::Type::Path(syn::TypePath{ref path,..}) = ty{
        if path.get_ident().is_some() && generic_types.contains(path.get_ident().unwrap()){
            return Some((*path).clone());
        }else{
            for segment in path.segments.iter(){
                if &segment.ident.to_string() == "PhantomData"{
                    continue;
                }
                if let syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments{ref args,..}) = &segment.arguments{
                    for arg in args.iter(){
                        if let syn::GenericArgument::Type(ref ty) = arg{
                            return  extract_generic_types(ty, generic_types);
                        }
                    }
                }
            }
            return None;
        }
    }
    None
}

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    
    let ident = input.ident;
    let ident_str = ident.to_string();

    let generics = input.generics;
    let (impl_generics, ty_generics, _) = generics.split_for_impl();
    
    let mut generic_types = Vec::new();

    let params = &generics.params;
    for param in params.iter(){
        if let syn::GenericParam::Type(ref type_param) = param{
            generic_types.push(type_param.ident.clone())
        }
    } 

    let fields  = if let syn::Data::Struct(syn::DataStruct{ref fields,..}) = input.data {
        fields
    }else{
        return TokenStream::from(syn::Error::new(ident.span(),"Custom Debug corresponds to Struct only").to_compile_error());
    };

    let mut fields_token = Vec::new();
    let mut generic_bound_types: Vec<syn::Path> = Vec::new();

    for field in fields.iter(){
        let pattern = extract_debug_pattern(field);
        let generic_type = extract_generic_types(&field.ty, &generic_types);
        if let Some(ref ty) = generic_type{
            if !generic_bound_types.contains(ty){
                generic_bound_types.push((*ty).clone());
            }
        } 
        
        match &field.ident{
            Some(name) => {
                let name_str = name.to_string();
                if pattern.is_some(){
                    fields_token.push(quote!{ .field(#name_str, &format_args!(#pattern, &self.#name)) });
                }else{
                    fields_token.push(quote!{ .field(#name_str, &self.#name) });
                }                
            }
            None => {}
        }
    };

    let generic_bound_tokens = generic_bound_types.iter().map(|bound_type|{
        quote!{#bound_type: ::std::fmt::Debug,}
    });

    let impl_token = if generic_bound_tokens.len() > 0 {
        quote!{impl #impl_generics ::std::fmt::Debug for #ident #ty_generics where #(#generic_bound_tokens)* }
    }else{
        quote!{impl #impl_generics ::std::fmt::Debug for #ident #ty_generics}
    }; 

    let expanded = quote!{
        #impl_token {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct(#ident_str)
                 #(#fields_token)*
                 .finish()
            }
        }
    };

    TokenStream::from(expanded)
}
