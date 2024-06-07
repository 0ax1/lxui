use std::sync::atomic::{AtomicU64, Ordering};

use lazy_static::lazy_static;
use vello::kurbo;

lazy_static! {
    static ref GLOBAL_UI_SCALE: AtomicU64 = AtomicU64::new(1.0f64.to_bits());
}

pub fn set_ui_scale(value: f64) {
    GLOBAL_UI_SCALE.store(value.to_bits(), Ordering::Relaxed);
}

pub fn ui_scale() -> f64 {
    f64::from_bits(GLOBAL_UI_SCALE.load(Ordering::Relaxed))
}

#[derive(Copy, Clone, Default)]
pub struct Context {
    pub location: kurbo::Point,
    pub level: i32,
}

pub trait AnyView: Draw + Layout + UserEvent + BaseFields + std::any::Any {
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait Draw {
    fn draw(&self, cx: Context, scene: &mut vello::Scene);
}

pub trait Layout {
    fn layout(&self, cx: Context);
}

pub trait UserEvent {
    fn mouse_down(&self, cx: Context);
}

pub trait BaseFields {
    fn rect(&self) -> kurbo::Rect;
    fn origin(&self) -> kurbo::Point;
    fn size(&self) -> kurbo::Size;
    fn width(&self) -> f64;
    fn height(&self) -> f64;
    fn visible(&self) -> bool;

    fn padding_top(&self) -> f64;
    fn padding_bottom(&self) -> f64;
    fn padding_left(&self) -> f64;
    fn padding_right(&self) -> f64;
    fn padding_vertical(&self) -> f64;
    fn padding_horizontal(&self) -> f64;

    fn on_click(&self) -> &Option<Box<dyn Fn()>>;
}

pub struct Base {
    pub size: std::cell::Cell<kurbo::Size>,
    pub origin: std::cell::Cell<kurbo::Point>,
    pub visible: bool,

    pub padding_top: f64,
    pub padding_bottom: f64,
    pub padding_left: f64,
    pub padding_right: f64,

    pub on_click: Option<Box<dyn Fn()>>,
}

impl Default for Base {
    fn default() -> Self {
        Self {
            size: std::cell::Cell::new(kurbo::Size::default()),
            origin: std::cell::Cell::new(kurbo::Point::default()),
            visible: true,

            padding_top: 0.0,
            padding_bottom: 0.0,
            padding_left: 0.0,
            padding_right: 0.0,

            on_click: None,
        }
    }
}

pub trait ViewSequence {
    fn into_view_sequence(self) -> Vec<Box<dyn AnyView>>;
}

impl ViewSequence for () {
    fn into_view_sequence(self) -> Vec<Box<dyn AnyView>> {
        vec![]
    }
}

impl<T: AnyView> ViewSequence for T {
    fn into_view_sequence(self) -> Vec<Box<dyn AnyView>> {
        vec![Box::new(self) as Box<dyn AnyView>]
    }
}

macro_rules! impl_into_view_sequence {
    ($( { $($idx:tt $T:ident),+ } ),+ ) => {
        $(
            impl<$($T: AnyView),+> ViewSequence for ($($T,)+) {
                fn into_view_sequence(self) -> Vec<Box<dyn AnyView>> {
                    vec![
                        $(Box::new(self.$idx) as Box<dyn AnyView>,)+
                    ]
                }
            }
        )+
    }
}

impl_into_view_sequence! {
    { 0 T0 },
    { 0 T0, 1 T1 },
    { 0 T0, 1 T1, 2 T2 },
    { 0 T0, 1 T1, 2 T2, 3 T3 },
    { 0 T0, 1 T1, 2 T2, 3 T3, 4 T4 },
    { 0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5 },
    { 0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6 },
    { 0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7 },
    { 0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8 },
    { 0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9 },
    { 0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10 },
    { 0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11 },
    { 0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11, 12 T12 },
    { 0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11, 12 T12, 13 T13 },
    { 0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11, 12 T12, 13 T13, 14 T14 },
    { 0 T0, 1 T1, 2 T2, 3 T3, 4 T4, 5 T5, 6 T6, 7 T7, 8 T8, 9 T9, 10 T10, 11 T11, 12 T12, 13 T13, 14 T14, 15 T15 }
}
