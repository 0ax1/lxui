pub struct CX;

pub use self::view::{View, Draw, ViewBase};

pub mod view {
    use super::*;

    pub trait View: Draw + ViewBase {}

    pub trait Draw {
        fn draw(&self, cx: &CX);
    }

    pub trait ViewBase {
        fn size(&self) -> Size;
    }

    #[derive(Debug, Copy, Clone, Default)]
    pub struct Position {
        pub x: i32,
        pub y: i32,
    }

    #[derive(Debug, Copy, Clone, Default)]
    pub struct Size {
        pub width: i32,
        pub height: i32,
    }

    #[derive(Copy, Clone, Default)]
    pub struct Base {
        pub size: Size,
        pub padding_top: i32,
        pub padding_bottom: i32,
        pub padding_left: i32,
        pub padding_right: i32,
    }

    impl std::fmt::Display for Position {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{{ x: {}, y: {} }}", self.x, self.y)
        }
    }

    impl std::fmt::Display for Size {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{{ width: {}, height: {} }}", self.width, self.height)
        }
    }

    pub trait Group {
        fn into_draw_group(self) -> Vec<Box<dyn View>>;
    }

    macro_rules! impl_into_view_group {
        ($( { $($idx:tt $T:ident),+ } ),+ ) => {
            $(
                impl<$($T: View + 'static),+> Group for ($($T,)+) {
                    fn into_draw_group(self) -> Vec<Box<dyn View>> {
                        vec![
                            $(Box::new(self.$idx) as Box<dyn View>,)+
                        ]
                    }
                }
            )+
        }
    }

    impl_into_view_group! {
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
}
