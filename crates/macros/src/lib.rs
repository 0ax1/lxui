use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ViewBase)]
pub fn derive_view_base(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expanded = quote! {
        impl #name {
            pub fn size(mut self, width: i32, height: i32) -> Self {
                self.view_base.size.width = width;
                self.view_base.size.height = height;
                self
            }

            pub fn visible(mut self, visible: bool) -> Self {
                self.view_base.visible = visible;
                self
            }

            pub fn padding_top(mut self, amount: i32) -> Self {
                self.view_base.padding_top = amount;
                self
            }

            pub fn padding_bottom(mut self, amount: i32) -> Self {
                self.view_base.padding_bottom = amount;
                self
            }

            pub fn padding_left(mut self, amount: i32) -> Self {
                self.view_base.padding_left = amount;
                self
            }

            pub fn padding_right(mut self, amount: i32) -> Self {
                self.view_base.padding_right = amount;
                self
            }

            pub fn padding_horizontal(mut self, padding: i32) -> Self {
                self.view_base.padding_left = padding;
                self.view_base.padding_right = padding;
                self
            }

            pub fn padding_vertical(mut self, padding: i32) -> Self {
                self.view_base.padding_top = padding;
                self.view_base.padding_bottom = padding;
                self
            }
        }

        impl ViewBase for #name {
            fn size(&self) -> view::Size {
                self.view_base.size
            }

            fn width(&self) -> i32 {
                self.view_base.size.width
            }

            fn height(&self) -> i32 {
                self.view_base.size.height
            }

            fn visible(&self) -> bool {
                self.view_base.visible
            }

            fn padding_top(&self) -> i32 {
                self.view_base.padding_top
            }

            fn padding_bottom(&self) -> i32 {
                self.view_base.padding_bottom
            }

            fn padding_left(&self) -> i32 {
                self.view_base.padding_left
            }

            fn padding_right(&self) -> i32 {
                self.view_base.padding_right
            }

            fn padding_horizontal(&self) -> i32 {
                self.view_base.padding_left + self.view_base.padding_right
            }

            fn padding_vertical(&self) -> i32 {
                self.view_base.padding_top + self.view_base.padding_bottom
            }
        }

        impl View for #name{}
    };

    TokenStream::from(expanded)
}
