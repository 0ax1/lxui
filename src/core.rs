use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use lazy_static::lazy_static;
use vello::kurbo;

use crate::state;

pub fn callback<T>(state: &State<T>, f: impl Fn(&mut T)) -> impl Fn()
where
    T: Clone + 'static,
{
    let state = state.clone();

    move || {
        f(&mut state.borrow_mut());
        state.notify();
    }
}

pub struct State<T: 'static + Clone> {
    data: Rc<RefCell<T>>,
    subscribers: Rc<RefCell<std::vec::Vec<Box<dyn FnMut(&mut T)>>>>,
}

impl<T: Clone + 'static> State<T> {
    pub fn new(value: T) -> Self {
        state::STATE_MANAGER.with(|manager| {
            let mut manager = manager.borrow_mut();
            if let Some(other) = manager.get_state::<T>(0) {
                return State {
                    data: Rc::new(RefCell::new(other)),
                    subscribers: Rc::new(RefCell::default()),
                };
            } else {
                return State {
                    data: Rc::new(RefCell::new(value)),
                    subscribers: Rc::new(RefCell::default()),
                };
            }
        })
    }

    pub fn value(&self) -> T {
        self.data.borrow().clone()
    }

    pub fn subscribe<F>(&self, closure: F)
    where
        F: FnMut(&mut T) + 'static,
    {
        self.subscribers.borrow_mut().push(Box::new(closure));
    }

    pub fn notify(&self) {
        for subscriber in self.subscribers.borrow_mut().iter_mut() {
            subscriber(&mut self.data.borrow_mut());
        }
    }
}

impl<T: 'static + Clone> Drop for State<T> {
    fn drop(&mut self) {
        state::STATE_MANAGER.with(|manager| {
            let mut manager = manager.borrow_mut();
            manager.set_state(0, self.data.clone().borrow().clone());
        })
    }
}

impl<T: Clone> std::clone::Clone for State<T> {
    fn clone(&self) -> Self {
        State {
            data: self.data.clone(),
            subscribers: self.subscribers.clone(),
        }
    }
}

impl<T: Clone> Deref for State<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Clone> DerefMut for State<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Rc::get_mut(&mut self.data).expect("error: multiple references")
    }
}

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

pub trait AnyView: Draw + Layout + UserEvent + ViewBase + std::any::Any {
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

pub trait ViewBase {
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
