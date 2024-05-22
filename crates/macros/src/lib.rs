use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(AnyView)]
pub fn derive_draw_size(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expanded = quote! {
        impl #name {
            pub fn size(mut self, size: Size) -> Self {
                self.view_base.size = size;
                self
            }
        }

        impl AnyView for #name {
            fn size(&self) -> Size {
                self.view_base.size
            }
        }
    };

    TokenStream::from(expanded)
}
