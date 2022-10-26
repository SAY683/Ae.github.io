/*
Property宏设置
 */
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

///#EnvironmentVariable宏
#[proc_macro_derive(SlimeEnvironment)]
pub fn file_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    return TokenStream::from(quote! {
        impl SlimeEnvironment for #name{}
    });
}
///#async_trait必须
#[proc_macro_derive(MysqlServer)]
pub fn mysql_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    return TokenStream::from(quote! {
        #[async_trait]
        impl MysqlServer for #name{}
    });
}
///#async_trait必须
#[proc_macro_derive(RedisServer)]
pub fn rides_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    return TokenStream::from(quote! {
        #[async_trait]
        impl RedisServer for #name{}
    });
}
