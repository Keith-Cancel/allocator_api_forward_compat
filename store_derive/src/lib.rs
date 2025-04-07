use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Store)]
pub fn derive_store(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        api_traits::macros::__impl_store!(#name);
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(StoreDangling)]
pub fn derive_store_dangling(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        api_traits::macros::__impl_store_dangling!(#name);
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(StoreStable)]
pub fn derive_store_stable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        api_traits::macros::__impl_store_stable!(#name);
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(StoreNoOpResolve)]
pub fn derive_store_no_op_resolve(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        api_traits::macros::__impl_store_no_op_resolve!(#name);
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(StorePinning)]
pub fn derive_store_pinning(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        api_traits::macros::__impl_store_pinning!(#name);
    };
    TokenStream::from(expanded)
}
