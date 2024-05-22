pub struct CX;

pub use self::view::{AnyView, Draw, ViewBase};

pub mod view {
    use super::*;

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

    pub trait AnyView: Draw + ViewBase {}

    pub trait Group {
        fn into_draw_group(self) -> Vec<Box<dyn AnyView>>;
    }

    impl<T1: AnyView + 'static> Group for (T1,) {
        fn into_draw_group(self) -> Vec<Box<dyn AnyView>> {
            vec![Box::new(self.0) as Box<dyn AnyView>]
        }
    }

    impl<T1: AnyView + 'static, T2: AnyView + 'static> Group for (T1, T2) {
        fn into_draw_group(self) -> Vec<Box<dyn AnyView>> {
            vec![
                Box::new(self.0) as Box<dyn AnyView>,
                Box::new(self.1) as Box<dyn AnyView>,
            ]
        }
    }

    impl<T1: AnyView + 'static, T2: AnyView + 'static, T3: AnyView + 'static> Group for (T1, T2, T3) {
        fn into_draw_group(self) -> Vec<Box<dyn AnyView>> {
            vec![
                Box::new(self.0) as Box<dyn AnyView>,
                Box::new(self.1) as Box<dyn AnyView>,
                Box::new(self.2) as Box<dyn AnyView>,
            ]
        }
    }

    impl<
            T1: AnyView + 'static,
            T2: AnyView + 'static,
            T3: AnyView + 'static,
            T4: AnyView + 'static,
        > Group for (T1, T2, T3, T4)
    {
        fn into_draw_group(self) -> Vec<Box<dyn AnyView>> {
            vec![
                Box::new(self.0) as Box<dyn AnyView>,
                Box::new(self.1) as Box<dyn AnyView>,
                Box::new(self.2) as Box<dyn AnyView>,
                Box::new(self.3) as Box<dyn AnyView>,
            ]
        }
    }
}
