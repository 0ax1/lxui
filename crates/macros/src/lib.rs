use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ViewSize)]
pub fn derive_draw_size(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let expanded = quote! {
        impl #name {
            pub fn size(mut self, size: Size) -> Self {
                self.size = size;
                self
            }
        }

        impl ViewSize for #name {
            fn size(&self) -> Size {
                self.size
            }
        }
    };

    TokenStream::from(expanded)
}
