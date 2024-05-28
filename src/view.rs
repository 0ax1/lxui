use crate::{core, *};
use macros::*;
use vello::{kurbo, peniko};

pub mod event {
    pub fn callback<T>(
        state: &std::rc::Rc<std::cell::RefCell<T>>,
        f: impl Fn(&mut T),
    ) -> impl Fn() {
        let state = state.clone();

        move || {
            f(&mut state.borrow_mut());
        }
    }
}

#[derive(AnyView)]
pub struct Loop {
    view_base: core::Base,
    elements: Vec<Box<dyn core::AnyView>>,
}

impl Loop {
    pub fn new<F, T>(range: std::ops::Range<u32>, func: F) -> Loop
    where
        T: AnyView,
        F: Fn(u32) -> T,
    {
        Loop {
            view_base: core::Base::default(),
            elements: range
                .map(|idx| Box::new(func(idx)) as Box<dyn AnyView>)
                .collect(),
        }
    }
}

impl core::Draw for Loop {
    fn draw(&self, _: core::Context, _: &mut vello::Scene) {}
}

pub trait Stack: ViewBase {
    fn elements(&self) -> &[Box<dyn core::AnyView>];

    fn recurse_stack(&self, mut operation: impl FnMut(&Box<dyn AnyView>)) {
        fn r(element: &Box<dyn AnyView>, operation: &mut impl FnMut(&Box<dyn AnyView>)) {
            if let Some(list) = element.as_any().downcast_ref::<Loop>() {
                for element in list.elements.iter().filter(|e| e.visible()) {
                    r(element, operation);
                }
            } else {
                operation(element);
            }
        }

        for element in self.elements().iter().filter(|e| e.visible()) {
            r(element, &mut operation);
        }
    }
}

#[derive(AnyView)]
pub struct VStack {
    view_base: core::Base,
    spacing: f64,
    elements: Vec<Box<dyn core::AnyView>>,
}

impl VStack {
    pub fn new<T: core::ViewSequence>(elements: T) -> VStack {
        VStack {
            view_base: core::Base::default(),
            elements: elements.into_view_sequence(),
            spacing: 0.0,
        }
    }

    pub fn spacing(mut self, distance: f64) -> Self {
        self.spacing = distance;
        self
    }
}

impl Stack for VStack {
    fn elements(&self) -> &[Box<dyn core::AnyView>] {
        &self.elements
    }
}

impl core::Draw for VStack {
    fn draw(&self, mut cx: core::Context, scene: &mut vello::Scene) {
        println!("L{} VStack {} {}", cx.level, self.size(), cx.origin);
        cx.level += 1;

        let process = |element: &Box<dyn AnyView>| {
            element.draw(
                core::Context {
                    // Apply the origin offset of the VStack itself.
                    origin: core::Origin {
                        x: cx.origin.x + self.padding_left(),
                        y: cx.origin.y + self.padding_top(),
                    },
                    ..cx
                },
                scene,
            );

            // Offset origin.y for the next element in the VStack.
            cx.origin.y += element.height();
            cx.origin.y += element.padding_vertical();
            cx.origin.y += self.spacing;
        };

        self.recurse_stack(process);
    }
}

#[derive(AnyView)]
pub struct HStack {
    view_base: core::Base,
    spacing: f64,
    elements: Vec<Box<dyn core::AnyView>>,
}

impl HStack {
    pub fn new<T: core::ViewSequence>(elements: T) -> HStack {
        HStack {
            view_base: core::Base::default(),
            spacing: 0.0,
            elements: elements.into_view_sequence(),
        }
    }

    pub fn spacing(mut self, distance: f64) -> Self {
        self.spacing = distance;
        self
    }
}

impl Stack for HStack {
    fn elements(&self) -> &[Box<dyn core::AnyView>] {
        &self.elements
    }
}

impl core::Draw for HStack {
    fn draw(&self, mut cx: core::Context, scene: &mut vello::Scene) {
        println!("L{} HStack {} {}", cx.level, self.size(), cx.origin);
        cx.level += 1;

        // Given that the root view is a container and always drawn,
        // only view containers need to check for element visibility.
        let process = |element: &Box<dyn AnyView>| {
            element.draw(
                core::Context {
                    // Apply the origin offset of the HStack itself.
                    origin: core::Origin {
                        x: cx.origin.x + self.padding_left(),
                        y: cx.origin.y + self.padding_top(),
                    },
                    ..cx
                },
                scene,
            );

            // Offset origin.x for the next element in the HStack.
            cx.origin.x += element.width();
            cx.origin.x += element.padding_horizontal();
            cx.origin.x += self.spacing;
        };

        self.recurse_stack(process);
    }
}

#[derive(AnyView)]
pub struct ZStack {
    view_base: core::Base,
    elements: Vec<Box<dyn core::AnyView>>,
}

impl ZStack {
    pub fn new<T: core::ViewSequence>(elements: T) -> ZStack {
        ZStack {
            view_base: core::Base::default(),
            elements: elements.into_view_sequence(),
        }
    }
}

impl Stack for ZStack {
    fn elements(&self) -> &[Box<dyn core::AnyView>] {
        &self.elements
    }
}

impl core::Draw for ZStack {
    fn draw(&self, mut cx: core::Context, scene: &mut vello::Scene) {
        println!("L{} ZStack {} {}", cx.level, self.size(), cx.origin);
        cx.level += 1;

        let process = |element: &Box<dyn AnyView>| {
            element.draw(
                core::Context {
                    // Apply the origin offset of the ZStack itself.
                    origin: core::Origin {
                        x: cx.origin.x + self.padding_left(),
                        y: cx.origin.y + self.padding_top(),
                    },
                    ..cx
                },
                scene,
            );
        };

        self.recurse_stack(process);
    }
}

enum Style {
    Fill(Color),
    Stroke(Color, f64),
}

impl Default for Style {
    fn default() -> Self {
        Style::Stroke(Color::rgb8(1, 1, 1), 1.0)
    }
}

#[derive(Default, AnyView)]
pub struct Rectangle {
    view_base: core::Base,
    fill: Option<peniko::Color>,
    stroke: Option<(peniko::Color, f64)>,
}

impl Rectangle {
    pub fn fill(mut self, color: Color) -> Self {
        self.fill = Some(color);
        self
    }

    pub fn stroke(mut self, color: Color, stroke_width: f64) -> Self {
        self.stroke = Some((color, stroke_width));
        self
    }
}

impl core::Draw for Rectangle {
    fn draw(&self, cx: core::Context, scene: &mut vello::Scene) {
        println!("L{} Rectangle {} {}", cx.level, self.size(), cx.origin);

        let rect = kurbo::Rect::new(
            cx.origin.x + self.padding_left(),
            cx.origin.y + self.padding_top(),
            cx.origin.x + self.padding_left() + self.width(),
            cx.origin.y + self.padding_top() + self.height(),
        );

        let _is_hovered = (rect.x0..=rect.x1).contains(&cx.cursor_position.x)
            && (rect.y0..=rect.y1).contains(&cx.cursor_position.y);

        if let Some(color) = self.fill {
            scene.fill(
                peniko::Fill::NonZero,
                kurbo::Affine::IDENTITY,
                color,
                None,
                &rect,
            );
        }

        if let Some((color, stroke_width)) = self.stroke {
            scene.stroke(
                &kurbo::Stroke::new(stroke_width).with_join(kurbo::Join::Miter),
                kurbo::Affine::IDENTITY,
                color,
                None,
                &rect,
            );
        }
    }
}

#[derive(Default, AnyView)]
pub struct Circle {
    view_base: core::Base,
    fill: Option<peniko::Color>,
    stroke: Option<(peniko::Color, f64)>,
}

impl Circle {
    pub fn diameter(self, diameter: f64) -> Self {
        self.size(diameter, diameter)
    }

    pub fn radius(self, radius: f64) -> Self {
        self.size(radius * 2.0, radius * 2.0)
    }

    pub fn fill(mut self, color: Color) -> Self {
        self.fill = Some(color);
        self
    }

    pub fn stroke(mut self, color: Color, stroke_width: f64) -> Self {
        self.stroke = Some((color, stroke_width));
        self
    }
}

impl core::Draw for Circle {
    fn draw(&self, cx: core::Context, scene: &mut vello::Scene) {
        println!("L{} Circle {} {}", cx.level, self.size(), cx.origin);

        let circle = vello::kurbo::Circle::new(
            (
                cx.origin.x + self.padding_left() + self.width() / 2.0,
                cx.origin.y + self.padding_top() + self.height() / 2.0,
            ),
            self.width() / 2.0,
        );

        if let Some(color) = self.fill {
            scene.fill(
                peniko::Fill::NonZero,
                kurbo::Affine::IDENTITY,
                color,
                None,
                &circle,
            );
        }

        if let Some((color, stroke_width)) = self.stroke {
            scene.stroke(
                &kurbo::Stroke::new(stroke_width),
                kurbo::Affine::IDENTITY,
                color,
                None,
                &circle,
            );
        }
    }
}
