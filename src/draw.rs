pub trait Draw {
    fn draw(&self);
    fn size_(&self) -> Size;
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

pub trait DrawGroup {
    fn into_vec(self) -> Vec<Box<dyn Draw>>;
}

impl<T1: Draw + 'static> DrawGroup for (T1,) {
    fn into_vec(self) -> Vec<Box<dyn Draw>> {
        vec![Box::new(self.0) as Box<dyn Draw>]
    }
}

impl<T1: Draw + 'static, T2: Draw + 'static> DrawGroup for (T1, T2) {
    fn into_vec(self) -> Vec<Box<dyn Draw>> {
        vec![
            Box::new(self.0) as Box<dyn Draw>,
            Box::new(self.1) as Box<dyn Draw>,
        ]
    }
}

impl<T1: Draw + 'static, T2: Draw + 'static, T3: Draw + 'static> DrawGroup for (T1, T2, T3) {
    fn into_vec(self) -> Vec<Box<dyn Draw>> {
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
    fn into_vec(self) -> Vec<Box<dyn Draw>> {
        vec![
            Box::new(self.0) as Box<dyn Draw>,
            Box::new(self.1) as Box<dyn Draw>,
            Box::new(self.2) as Box<dyn Draw>,
            Box::new(self.3) as Box<dyn Draw>,
        ]
    }
}
