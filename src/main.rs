mod draw;
use draw::*;

use macros::*;

#[derive(ViewBase)]
pub struct VStack {
    view_base: view::Base,
    spacing: i32,
    elements: Vec<Box<dyn view::View>>,
}

impl VStack {
    pub fn spacing(mut self, distance: i32) -> Self {
        self.spacing = distance;
        println!("spacing: {}", distance);
        self
    }
}

impl view::Draw for VStack {
    fn draw(&self, cx: &CX) {
        for element in &self.elements {
            if element.visible() {
                element.draw(cx)
            }
        }
    }
}

pub fn vstack<T: view::Group>(elements: T) -> VStack {
    VStack {
        view_base: view::Base::default(),
        spacing: i32::default(),
        elements: elements.into_view_group(),
    }
}

#[derive(Default, ViewBase)]
pub struct Rectangle {
    view_base: view::Base,
}

impl view::Draw for Rectangle {
    fn draw(&self, _cx: &CX) {
        println!("rectangle: {}", self.size());
    }
}

#[derive(Default, ViewBase)]
pub struct Circle {
    view_base: view::Base,
}

impl view::Draw for Circle {
    fn draw(&self, _cx: &CX) {
        println!("circle: {}", self.size());
    }
}

#[rustfmt::skip]
fn canvas() -> impl view::Draw {
    vstack((
        Rectangle::default()
            .size(100, 100),

        Circle::default()
            .size(100, 100)
            .visible(false),

        vstack((
            Rectangle::default()
                .size(200, 200)
                .padding_vertical(10, 10),

            Circle::default()
                .size(200, 200)
                .padding_horizontal(10, 10),
        ))
        .spacing(5)
    ))
    .spacing(10)
    .size(400, 400)
}

fn main() {
    let cx = CX;
    canvas().draw(&cx);
}
