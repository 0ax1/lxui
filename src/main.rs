mod draw;
use draw::*;

impl Draw for VStack {
    fn size_(&self) -> Size {
        Size {
            width: 100,
            height: 100,
        }
    }

    fn draw(&self) {
        for element in &self.elements {
            element.draw()
        }
    }
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

pub fn vstack<T: DrawGroup>(elements: T) -> VStack {
    VStack {
        size: Size::default(),
        elements: elements.into_vec(),
    }
}

#[derive(Copy, Clone, Default)]
pub struct Rectangle {
    size: Size,
}

fn rectangle() -> Rectangle {
    Rectangle::default()
}

impl Rectangle {
    fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl Draw for Rectangle {
    fn size_(&self) -> Size {
        self.size
    }

    fn draw(&self) {
        println!("rectangle: {}", self.size_());
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

fn circle() -> Circle {
    Circle::default()
}

impl Draw for Circle {
    fn size_(&self) -> Size {
        Size {
            width: 100,
            height: 100,
        }
    }

    fn draw(&self) {
        println!("circle: {}", self.size_());
    }
}

pub struct VStack {
    size: Size,
    elements: Vec<Box<dyn Draw>>,
}

impl VStack {
    pub fn draw(&self) {
        for element in &self.elements {
            element.draw();
        }
    }

    fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn spacing(self, distance: i32) -> Self {
        println!("spacing: {}", distance);
        self
    }
}

#[rustfmt::skip]
fn canvas() -> impl Draw {
    vstack((
        rectangle().size(Size {width: 100, height: 100}), 
        circle().size(Size {width: 100, height: 100}),

        vstack((
            rectangle(), 
            circle(),
        ))
        .spacing(5)

    ))
    .spacing(10)
    .size(Size {width: 400, height: 400})
}

fn main() {
    canvas().draw();
}
