use proc_macro::TokenStream;

extern crate proc_macro;

#[proc_macro]
pub fn impl_coords(_: TokenStream) -> TokenStream {
    "fn pos_x(&mut self) -> &mut i32 { &mut self.pos_x }
fn pos_y(&mut self) -> &mut i32 { &mut self.pos_y }"
        .parse()
        .unwrap()
}
