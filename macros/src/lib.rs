use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(AnyView)]
pub fn derive_view_base(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let mut expanded = quote! {
        impl #name {
            pub fn size(mut self, width: f64, height: f64) -> Self {
                self.view_base.size.set(vello::kurbo::Size { width, height });
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

            pub fn on_click(mut self, on_click: impl Fn() + 'static) -> Self {
                self.view_base.on_click = Some(Box::new(on_click));
                self
            }
        }

        impl ViewBase for #name {
            fn rect(&self) -> vello::kurbo::Rect {
                let origin = self.view_base.origin.get();
                vello::kurbo::Rect {
                    x0: origin.x,
                    y0: origin.y,
                    x1: origin.x + self.width(),
                    y1: origin.y + self.height(),
                }
            }

            fn size(&self) -> vello::kurbo::Size {
                vello::kurbo::Size {
                    width: self.view_base.size.get().width * ui_scale(),
                    height: self.view_base.size.get().height * ui_scale()
                }
            }

            fn origin(&self) -> vello::kurbo::Point {
                self.view_base.origin.get()
            }

            fn width(&self) -> f64 {
                self.view_base.size.get().width * ui_scale()
            }

            fn height(&self) -> f64 {
                self.view_base.size.get().height * ui_scale()
            }

            fn visible(&self) -> bool {
                self.view_base.visible
            }

            fn padding_top(&self) -> f64 {
                self.view_base.padding_top * ui_scale()
            }

            fn padding_bottom(&self) -> f64 {
                self.view_base.padding_bottom * ui_scale()
            }

            fn padding_left(&self) -> f64 {
                self.view_base.padding_left * ui_scale()
            }

            fn padding_right(&self) -> f64 {
                self.view_base.padding_right * ui_scale()
            }

            fn padding_horizontal(&self) -> f64 {
                self.view_base.padding_left * ui_scale()
                + self.view_base.padding_right * ui_scale()
            }

            fn padding_vertical(&self) -> f64 {
                self.view_base.padding_top * ui_scale()
                + self.view_base.padding_bottom * ui_scale()
            }

            fn on_click(&self) -> &Option<Box<dyn Fn()>> {
                &self.view_base.on_click
            }
        }

        impl core::AnyView for #name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };

    if ["VStack", "HStack", "ZStack"].contains(&name.to_string().as_str()) {
        let expanded2 = quote! {
            impl core::UserEvent for #name {
                fn mouse_down(&self, cx: core::Context) {
                    let rect = self.rect();
                    if (rect.x0..=rect.x1).contains(&cx.location.x) &&
                       (rect.y0..=rect.y1).contains(&cx.location.y) {

                       self.recurse_stack(|element: &Box<dyn AnyView>| {
                           element.mouse_down(cx);
                       });

                       if let Some(action) = self.on_click() {
                           action();
                       }
                    }
                }
            }
        };

        expanded.extend(expanded2);
    } else {
        let expanded2 = quote! {
            impl core::Layout for #name {
                fn layout(&self, cx: Context) {
                    self.view_base.origin.set(
                        vello::kurbo::Point {
                            x: cx.location.x + self.padding_left(),
                            y: cx.location.y + self.padding_top(),
                        }
                    )
                }
            }

            impl core::UserEvent for #name {
                fn mouse_down(&self, cx: core::Context) {
                    if let Some(action) = self.on_click() {
                        let rect = self.rect();
                        if (rect.x0..=rect.x1).contains(&cx.location.x) &&
                           (rect.y0..=rect.y1).contains(&cx.location.y) {
                            action();
                        }
                    }
                }
            }
        };

        expanded.extend(expanded2);
    }

    TokenStream::from(expanded)
}
