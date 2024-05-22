pub struct CX;

pub trait Draw {
    fn draw(&self, cx: &CX);
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

pub trait DrawGroup {
    fn into_draw_group(self) -> Vec<Box<dyn Draw>>;
}

impl<T1: Draw + 'static> DrawGroup for (T1,) {
    fn into_draw_group(self) -> Vec<Box<dyn Draw>> {
        vec![Box::new(self.0) as Box<dyn Draw>]
    }
}

impl<T1: Draw + 'static, T2: Draw + 'static> DrawGroup for (T1, T2) {
    fn into_draw_group(self) -> Vec<Box<dyn Draw>> {
        vec![
            Box::new(self.0) as Box<dyn Draw>,
            Box::new(self.1) as Box<dyn Draw>,
        ]
    }
}

impl<T1: Draw + 'static, T2: Draw + 'static, T3: Draw + 'static> DrawGroup for (T1, T2, T3) {
    fn into_draw_group(self) -> Vec<Box<dyn Draw>> {
        vec![
            Box::new(self.0) as Box<dyn Draw>,
            Box::new(self.1) as Box<dyn Draw>,
            Box::new(self.2) as Box<dyn Draw>,
        ]
    }
}

impl<T1: Draw + 'static, T2: Draw + 'static, T3: Draw + 'static, T4: Draw + 'static> DrawGroup
    for (T1, T2, T3, T4)
{
    fn into_draw_group(self) -> Vec<Box<dyn Draw>> {
        vec![
            Box::new(self.0) as Box<dyn Draw>,
            Box::new(self.1) as Box<dyn Draw>,
            Box::new(self.2) as Box<dyn Draw>,
            Box::new(self.3) as Box<dyn Draw>,
        ]
    }
}
