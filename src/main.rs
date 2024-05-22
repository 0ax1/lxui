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
            element.draw(cx)
        }
    }
}

pub fn vstack<T: view::Group>(elements: T) -> VStack {
    VStack {
        view_base: view::Base::default(),
        spacing: i32::default(),
        elements: elements.into_draw_group(),
    }
}

#[derive(Copy, Clone, Default, ViewBase)]
pub struct Rectangle {
    view_base: view::Base,
}

impl view::Draw for Rectangle {
    fn draw(&self, _cx: &CX) {
        println!("rectangle: {}", self.size());
    }
}

#[derive(Copy, Clone, Default, ViewBase)]
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
            .size(view::Size {width: 100, height: 100}), 

        Circle::default()
            .size(view::Size {width: 100, height: 100}),

        vstack((
            Rectangle::default()
                .size(view::Size {width: 200, height: 200}),

            Circle::default()
                .size(view::Size {width: 200, height: 200}),
        ))
        .spacing(5)
    ))
    .spacing(10)
    .size(view::Size {width: 400, height: 400})
}

fn main() {
    let cx = CX;
    canvas().draw(&cx);
}
