#![no_std]
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

extern crate proc_macro;

#[proc_macro_derive(Coordinates)]
pub fn derive_coords(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl Coords for #name {
            fn pos_x(&mut self) -> &mut i32 { &mut self.pos_x }
            fn pos_y(&mut self) -> &mut i32 { &mut self.pos_y }
        }
    };

    TokenStream::from(expanded)
}
