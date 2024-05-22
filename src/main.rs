mod draw;
use draw::*;

pub struct VStack {
    size: Size,
    elements: Vec<Box<dyn Draw>>,
}

impl VStack {
    fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn spacing(self, distance: i32) -> Self {
        println!("spacing: {}", distance);
        self
    }
}

impl Draw for VStack {
    fn size(&self) -> Size {
        self.size
    }

    fn draw(&self, cx: &CX) {
        for element in &self.elements {
            element.draw(cx)
        }
    }
}

pub fn vstack<T: DrawGroup>(elements: T) -> VStack {
    VStack {
        size: Size::default(),
        elements: elements.into_draw_group(),
    }
}

#[derive(Copy, Clone, Default)]
pub struct Rectangle {
    size: Size,
}

impl Rectangle {
    fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl Draw for Rectangle {
    fn size(&self) -> Size {
        self.size
    }

    fn draw(&self, _cx: &CX) {
        println!("rectangle: {}", self.size());
    }
}

#[derive(Copy, Clone, Default)]
pub struct Circle {
    size: Size,
}

impl Circle {
    fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl Draw for Circle {
    fn size(&self) -> Size {
        self.size
    }

    fn draw(&self, _cx: &CX) {
        println!("circle: {}", self.size());
    }
}

#[rustfmt::skip]
fn canvas() -> impl Draw {
    vstack((
        Rectangle::default()
            .size(Size {width: 100, height: 100}), 

        Circle::default()
            .size(Size {width: 100, height: 100}),

        vstack((
            Rectangle::default()
                .size(Size {width: 200, height: 200}),

            Circle::default()
                .size(Size {width: 200, height: 200}),
        ))
        .spacing(5)

    ))
    .spacing(10)
    .size(Size {width: 400, height: 400})
}

fn main() {
    let cx = CX;
    canvas().draw(&cx);
}
