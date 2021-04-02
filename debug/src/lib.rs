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

    let generic_bounds = generic_types.iter().map(|name| {
        quote!{ #name: ::std::fmt::Debug, }
    });

    let impl_token = if generic_bounds.len() > 0 {
        quote!{impl #impl_generics ::std::fmt::Debug for #ident #ty_generics where #(#generic_bounds)* }
    }else{
        quote!{impl #impl_generics ::std::fmt::Debug for #ident #ty_generics}
    };  

    let fields  = if let syn::Data::Struct(syn::DataStruct{ref fields,..}) = input.data {
        fields
    }else{
        return TokenStream::from(syn::Error::new(ident.span(),"Custom Debug corresponds to Struct only").to_compile_error());
    };

    let mut fields_token = Vec::new();

    for field in fields.iter(){
        let pattern = extract_debug_pattern(field);
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
