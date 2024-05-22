mod draw;
use draw::*;

use macros::AnyView;

#[derive(AnyView)]
pub struct VStack {
    view_base: ViewBase,
    spacing: i32,
    elements: Vec<Box<dyn Draw>>,
}

impl VStack {
    pub fn spacing(mut self, distance: i32) -> Self {
        self.spacing = distance;
        println!("spacing: {}", distance);
        self
    }
}

impl Draw for VStack {
    fn draw(&self, cx: &CX) {
        for element in &self.elements {
            element.draw(cx)
        }
    }
}

pub fn vstack<T: DrawGroup>(elements: T) -> VStack {
    VStack {
        view_base: ViewBase::default(),
        spacing: i32::default(),
        elements: elements.into_draw_group(),
    }
}

#[derive(Copy, Clone, Default, AnyView)]
pub struct Rectangle {
    view_base: ViewBase,
}

impl Draw for Rectangle {
    fn draw(&self, _cx: &CX) {
        println!("rectangle: {}", self.size());
    }
}

#[derive(Copy, Clone, Default, AnyView)]
pub struct Circle {
    view_base: ViewBase,
}

impl Draw for Circle {
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
