use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ViewBase)]
pub fn derive_draw_size(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expanded = quote! {
        impl #name {
            pub fn size(mut self, size: view::Size) -> Self {
                self.view_base.size = size;
                self
            }
        }

        impl ViewBase for #name {
            fn size(&self) -> view::Size {
                self.view_base.size
            }
        }

        impl View for #name{}
    };

    TokenStream::from(expanded)
}
