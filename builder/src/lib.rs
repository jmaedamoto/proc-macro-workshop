use proc_macro;
use proc_macro2::TokenStream;
use syn::{parse_macro_input, DeriveInput,Ident,Data, DataStruct, Fields, FieldsNamed};
use quote::{quote,format_ident};

fn extract_fields(data: &Data) -> &FieldsNamed{
    let fields = if let Data::Struct(DataStruct{
        fields: Fields::Named(ref fields
        ),
        ..
    }) = data{
        fields
    }else{
        unimplemented!();
    };
    fields
}

fn builder_setters(data: &Data) -> TokenStream{
    let fields = extract_fields(data);

    enum AttrParseResult {
        Value(String),
        InvalidKey(syn::Meta),
    }

    let setters = fields.named.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let attrs = &f.attrs;
        let each_lit = attrs.iter().find_map(|attr| match attr.parse_meta(){
            Ok(meta) => match meta{
                syn::Meta::List(syn::MetaList{
                    ref path,
                    paren_token: _,
                    ref nested,
                }) => {
                    path.get_ident().map(|i| i == "builder")?;
                    nested.first().and_then(|nm|  match nm{
                        syn::NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue{
                            ref path,
                            eq_token: _,
                            lit: syn::Lit::Str(ref litstr),
                        })) => {
                            if path.get_ident().unwrap() != "each"{
                                Some(AttrParseResult::InvalidKey(meta.clone()))
                            }else{
                                Some(AttrParseResult::Value(litstr.value()))
                            }
                        },
                        _ => None,
                    })
                },
                _ => None,
            },
            _ => None,
        });

        if let Some(AttrParseResult::InvalidKey(ref meta)) = each_lit {
            return syn::Error::new_spanned(meta, "expected `builder(each = \"...\")`").to_compile_error();
        }

        match each_lit {
            Some(AttrParseResult::Value(ref lit)) => {
                let ty = extract_inner_type(ty).unwrap()[0];
                let lit = format_ident!("{}",lit);
                quote!{        
                    fn #lit(&mut self, #lit: #ty) -> &mut Self {
                        if self.#name.is_none(){
                            self.#name = Some(Vec::new());
                        }
                        let vec = self.#name.as_mut().unwrap();
                        vec.push(#lit);
                        self
                    }
                }
            },
            _ => {
                if is_option(ty){
                    let ty = extract_inner_type(ty).unwrap()[0];
                    quote!{        
                        fn #name(&mut self, #name: #ty) -> &mut Self {
                            self.#name = Some(#name);
                            self
                        }
                    }
                }else{
                    quote!{        
                        fn #name(&mut self, #name: #ty) -> &mut Self {
                            self.#name = Some(#name);
                            self
                        }
                    }
                }
            }
        }
    });
    quote! {#(#setters)*}
}

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input =  parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let builder_name = Ident::new(&format!("{}Builder",ident),ident.span());
    let fields = extract_fields(&input.data);

    let builder_fields = fields.named.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        if is_option(ty){
            quote!{ #name: #ty }
        }else{
            quote!{ #name: std::option::Option<#ty> }
        }
    });

    let builder_empty = fields.named.iter().map(|f|{
        let name = &f.ident;
        let ty = &f.ty;
        if is_vec(ty){
            quote!{ #name: Some(Vec::new()) }
        }else{
            quote!{ #name: None }
        }
    });

    let builder_setters = builder_setters(&input.data);

    let builder_values = fields.named.iter().map(|f|{
        let name = &f.ident;
        let ty = &f.ty;
        if is_option(ty){
            quote!{#name: self.#name.clone()}
        }else{
            quote!{#name: self.#name.clone().unwrap()}
        }
    });

    let set_check = fields.named.iter().map(|f|{
        let name = &f.ident;
        let ty = &f.ty;
        let err = format!("field `{}` is not set.", name.as_ref().unwrap());
        if !is_option(ty){
            quote!{
                if self.#name.is_none(){
                    return Err(#err.into());
                }
            }
        }else{
            quote!{}
        }
    });

    let expanded = quote!{
        pub struct #builder_name {
            #(#builder_fields,)*
        }

        impl #builder_name {
            #builder_setters
            
            pub fn build(&mut self) -> std::result::Result<#ident, std::boxed::Box<dyn std::error::Error>> {
                #(#set_check)*
                Ok(#ident{
                    #(#builder_values,)*
                })
            }
        }

        impl #ident {
            pub fn builder() -> #builder_name{
                #builder_name {
                    #(#builder_empty,)*
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn is_type(ident: &str, ty: &syn::Type) -> bool{
    if let syn::Type::Path(syn::TypePath {ref path, ..}) = ty {
        if !path.segments.is_empty() && path.segments.last().unwrap().ident == ident{
            return true;
        }
    }
    false
}

fn is_vec(ty: &syn::Type) -> bool {
    is_type("Vec", ty)
}

fn is_option(ty: &syn::Type) -> bool {
    is_type("Option", ty)
}

fn extract_inner_type(ty: &syn::Type) -> std::option::Option<Vec<&syn::Type>>{
    if let syn::Type::Path( syn::TypePath{ref path,..}) = ty{
        if !path.segments.is_empty(){
            if let syn::PathArguments::AngleBracketed(ref bracketed_generics) = path.segments.last().unwrap().arguments{
                let mut ty_vec = Vec::new();
                for generic in bracketed_generics.args.iter(){
                    if let syn::GenericArgument::Type(ref ty) = generic{
                        ty_vec.push(ty);
                    }
                }

                if !ty_vec.is_empty(){
                    return Some(ty_vec);
                }
            }
        }
    }
    None
}