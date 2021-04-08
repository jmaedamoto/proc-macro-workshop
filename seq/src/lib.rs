use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use syn::{parse_macro_input, Ident, LitInt, Token, braced};
use syn::parse::Parse;
use std::ops::Range;
use quote::{quote, format_ident};

struct Item {
    ident: Ident,
    range: Range<i128>,
    content: TokenStream2,
}

fn contains_loop(input: &TokenStream2) -> bool {
    let input: Vec<TokenTree> = input
        .clone()
        .into_iter()
        .map(|item| item)
        .collect();
    
    let mut iter = input.iter().enumerate();
    while let Some((i,token_tree)) = iter.next(){
        match &token_tree{
            TokenTree::Punct(punct) if punct.as_char() == '#' => {
                match &input.get(i + 1){
                    Some(TokenTree::Group(group)) => {
                        if let proc_macro2::Delimiter::Parenthesis = group.delimiter(){
                            match &input.get(i + 2){
                                Some(TokenTree::Punct(punct)) if punct.as_char() == '*' => {
                                    return true
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            TokenTree::Group(group) => {
                if contains_loop(&group.stream()){
                    return true
                }
            }
            _ => {}
        }
    }
    false
}

fn parse(input: &TokenStream2, ident: &Ident, range: &Range<i128>, counter: &i128) -> TokenStream2{
    let mut token_stream: Vec<TokenStream2> = Vec::new();
    let input: Vec<TokenTree> = input
        .clone()
        .into_iter()
        .map(|item| item)
        .collect();
    let mut iter = input.iter().enumerate();

    while let Some((i,token_tree)) = iter.next(){
        match &token_tree{
            TokenTree::Punct(punct) if punct.as_char() == '#' => {
                match &input.get(i + 1){
                    Some(TokenTree::Ident(next_ident)) if *next_ident == *ident => {
                        let mut prev_ident: Option<&syn::Ident> = None;
                        if i > 0 {
                            if let Some(TokenTree::Ident(pi)) = &input.get(i - 1){
                                prev_ident = Some(pi);
                            }
                        }
                        match prev_ident{
                            Some(pi) => {
                                let ident = format_ident!("{}{}",pi,counter.to_string());
                                token_stream.pop();
                                token_stream.push(quote!{#ident});

                            }
                            None => {
                                let lit = proc_macro2::Literal::i128_unsuffixed(*counter);
                                token_stream.push(quote!{#lit});
                            }
                        }
                        iter.next();
                    }
                    Some(TokenTree::Group(group)) => {
                        if let proc_macro2::Delimiter::Parenthesis = group.delimiter(){
                            match &input.get(i + 2){
                                Some(TokenTree::Punct(punct)) if punct.as_char() == '*' => {
                                    for counter in range.clone(){
                                        let parsed_group = parse(&group.stream(), ident, range, &counter);
                                        token_stream.push(quote!{#parsed_group});
                                        iter.next();
                                        iter.next();
                                    }
                                },
                                _ => {
                                    token_stream.push(quote!{#punct});
                                }
                            }
                        }else{
                            token_stream.push(quote!{#punct});
                        }
                    }
                    _ => {
                        token_stream.push(quote!{#punct});
                    }
                }
            }
            TokenTree::Ident(number_ident) if *number_ident == * ident => {
                let lit = proc_macro2::Literal::i128_unsuffixed(*counter);
                token_stream.push(quote!{#lit});
            }
            TokenTree::Group(group) => {
                let parsed_token = parse(&group.stream(),&ident,&range,&counter);
                let parsed_group = proc_macro2::Group::new(group.delimiter(), parsed_token);
                token_stream.push(quote!{#parsed_group});
            }
            tt => {
                token_stream.push(quote!{#tt});   
            }
        }
    }
    quote!{#(#token_stream)*}
}

impl Parse for Item {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        input.parse::<Token![in]>()?;
        let i : LitInt = input.parse()?;
        let i = i.base10_parse::<i128>()?;
        input.parse::<Token![..]>()?;
        let j: i128;
        if input.peek(Token![=]){
            input.parse::<Token![=]>()?;
            let j_lit : LitInt = input.parse()?;
            j = j_lit.base10_parse::<i128>()? + 1;
        }else{
            let j_lit : LitInt = input.parse()?;
            j = j_lit.base10_parse::<i128>()?;
        }
        let content: syn::parse::ParseBuffer;
        let _braces = braced!(content in input);
        let content: TokenStream2 = content.parse()?;
        let range = i..j;
        Ok(Item{
            ident,
            range,
            content,
        })
    }
}

#[proc_macro]
pub fn seq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Item);
    let content = &input.content;
    let ident = &input.ident;
    let range = &input.range;
    if contains_loop(&content){
        TokenStream::from(parse(&content,&ident,&range ,&0i128))
    }else{
        let parsed_token = range.clone().map(|counter|{
            parse(&content,&ident,&range ,&counter)
        });
        TokenStream::from(quote!{#(#parsed_token)*}) 
    }
}