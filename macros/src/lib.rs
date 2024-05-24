use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ViewBase)]
pub fn derive_view_base(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expanded = quote! {
        impl #name {
            pub fn size(mut self, width: f64, height: f64) -> Self {
                self.view_base.size.width = width;
                self.view_base.size.height = height;
                self
            }

            pub fn visible(mut self, visible: bool) -> Self {
                self.view_base.visible = visible;
                self
            }

            pub fn padding_top(mut self, amount: f64) -> Self {
                self.view_base.padding_top = amount;
                self
            }

            pub fn padding_bottom(mut self, amount: f64) -> Self {
                self.view_base.padding_bottom = amount;
                self
            }

            pub fn padding_left(mut self, amount: f64) -> Self {
                self.view_base.padding_left = amount;
                self
            }

            pub fn padding_right(mut self, amount: f64) -> Self {
                self.view_base.padding_right = amount;
                self
            }

            pub fn padding_horizontal(mut self, padding: f64) -> Self {
                self.view_base.padding_left = padding;
                self.view_base.padding_right = padding;
                self
            }

            pub fn padding_vertical(mut self, padding: f64) -> Self {
                self.view_base.padding_top = padding;
                self.view_base.padding_bottom = padding;
                self
            }
        }

        impl ViewBase for #name {
            fn size(&self) -> view::Size {
                self.view_base.size
            }

            fn width(&self) -> f64 {
                self.view_base.size.width
            }

            fn height(&self) -> f64 {
                self.view_base.size.height
            }

            fn visible(&self) -> bool {
                self.view_base.visible
            }

            fn padding_top(&self) -> f64 {
                self.view_base.padding_top
            }

            fn padding_bottom(&self) -> f64 {
                self.view_base.padding_bottom
            }

            fn padding_left(&self) -> f64 {
                self.view_base.padding_left
            }

            fn padding_right(&self) -> f64 {
                self.view_base.padding_right
            }

            fn padding_horizontal(&self) -> f64 {
                self.view_base.padding_left + self.view_base.padding_right
            }

            fn padding_vertical(&self) -> f64 {
                self.view_base.padding_top + self.view_base.padding_bottom
            }
        }

        impl view::View for #name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }

    };

    TokenStream::from(expanded)
}
