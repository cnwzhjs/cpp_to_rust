extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Layoutable)]
pub fn layoutable_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_layoutable_macro(&ast)
}

fn impl_layoutable_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Layoutable for #name {
            fn position(&self) -> Vector2f { self.layout.pos }
            fn size(&self) -> Vector2f { self.layout.size }
            fn set_position(&mut self, x: f32, y: f32) { self.layout.pos = Vector2f::new(x, y); }
            fn set_size(&mut self, width: f32, height: f32) { self.layout.size = Vector2f::new(width, height); }
        }
    };
    gen.into()
}

#[proc_macro_derive(Dockable)]
pub fn dockable_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_dockable_macro(&ast)
}

fn impl_dockable_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Dockable for #name {}
    };
    gen.into()
}