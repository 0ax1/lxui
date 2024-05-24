use macros::*;
use crate::{view, *};

#[derive(ViewBase)]
pub struct VStack {
    view_base: view::Base,
    spacing: i32,
    elements: Vec<Box<dyn view::View>>,
}

impl VStack {
    pub fn new<T: view::Group>(elements: T) -> VStack {
        VStack {
            view_base: view::Base::default(),
            elements: elements.into_view_group(),
            spacing: i32::default(),
        }
    }

    pub fn spacing(mut self, distance: i32) -> Self {
        self.spacing = distance;
        self
    }
}

impl view::Draw for VStack {
    fn draw(&self, mut cx: view::Context) {
        println!("L{} VStack {} {}", cx.level, self.size(), cx.origin);
        cx.level += 1;

        // Given that the root view is a container and always drawn,
        // only view containers need to check for element visibility.
        for element in self.elements.iter().filter(|e| e.visible()) {
            element.draw(cx);
            cx.origin.y += element.height();
            cx.origin.y += element.padding_vertical();
            cx.origin.y += self.spacing;
        }
    }
}

#[derive(ViewBase)]
pub struct HStack {
    view_base: view::Base,
    spacing: i32,
    elements: Vec<Box<dyn view::View>>,
}

impl HStack {
    pub fn new<T: view::Group>(elements: T) -> HStack {
        HStack {
            view_base: view::Base::default(),
            spacing: i32::default(),
            elements: elements.into_view_group(),
        }
    }

    pub fn spacing(mut self, distance: i32) -> Self {
        self.spacing = distance;
        self
    }
}

impl view::Draw for HStack {
    fn draw(&self, mut cx: view::Context) {
        println!("L{} HStack {} {}", cx.level, self.size(), cx.origin);
        cx.level += 1;

        // Given that the root view is a container and always drawn,
        // only view containers need to check for element visibility.
        for element in self.elements.iter().filter(|e| e.visible()) {
            element.draw(cx);
            cx.origin.x += element.width();
            cx.origin.x += element.padding_horizontal();
            cx.origin.x += self.spacing;
        }
    }
}

#[derive(Default, ViewBase)]
pub struct Rectangle {
    view_base: view::Base,
}

impl view::Draw for Rectangle {
    fn draw(&self, cx: view::Context) {
        println!("L{} Rectangle {} {}", cx.level, self.size(), cx.origin);
    }
}

#[derive(Default, ViewBase)]
pub struct Circle {
    view_base: view::Base,
}

impl view::Draw for Circle {
    fn draw(&self, cx: view::Context) {
        println!("L{} Circle {} {}", cx.level, self.size(), cx.origin);
    }
}

