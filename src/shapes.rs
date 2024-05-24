use crate::{view, *};
use macros::*;
use vello::peniko::Color;

#[derive(ViewBase)]
pub struct Loop {
    view_base: view::Base,
    elements: Vec<Box<dyn view::View>>,
}

impl Loop {
    pub fn new<F, T>(count: i32, func: F) -> Loop
    where
        T: View + 'static,
        F: Fn(i32) -> T,
    {
        Loop {
            view_base: view::Base::default(),
            elements: (0..count)
                .map(|idx| Box::new(func(idx)) as Box<dyn View>)
                .collect(),
        }
    }
}

impl view::Draw for Loop {
    fn draw(&self, _: view::Context, _: &mut vello::Scene) {}
}

#[derive(ViewBase)]
pub struct VStack {
    view_base: view::Base,
    spacing: f64,
    elements: Vec<Box<dyn view::View>>,
}

impl VStack {
    pub fn new<T: view::Group>(elements: T) -> VStack {
        VStack {
            view_base: view::Base::default(),
            elements: elements.into_view_group(),
            spacing: 0.0,
        }
    }

    pub fn spacing(mut self, distance: f64) -> Self {
        self.spacing = distance;
        self
    }
}

impl view::Draw for VStack {
    fn draw(&self, mut cx: view::Context, scene: &mut vello::Scene) {
        println!("L{} VStack {} {}", cx.level, self.size(), cx.origin);
        cx.level += 1;

        let process =
            |element: &Box<dyn View>, cx: &mut view::Context, scene: &mut vello::Scene| {
                element.draw(
                    view::Context {
                        origin: view::Origin {
                            x: cx.origin.x + self.padding_left(),
                            y: cx.origin.y + self.padding_top(),
                        },
                        ..*cx
                    },
                    scene,
                );
                cx.origin.y += element.height();
                cx.origin.y += element.padding_vertical();
                cx.origin.y += self.spacing;
            };

        // Given that the root view is a container and always drawn,
        // only view containers need to check for element visibility.
        for element in self.elements.iter().filter(|e| e.visible()) {
            if let Some(list) = element.as_any().downcast_ref::<Loop>() {
                for element in &list.elements {
                    process(element, &mut cx, scene);
                }
            } else {
                process(element, &mut cx, scene);
            }
        }
    }
}

#[derive(ViewBase)]
pub struct HStack {
    view_base: view::Base,
    spacing: f64,
    elements: Vec<Box<dyn view::View>>,
}

impl HStack {
    pub fn new<T: view::Group>(elements: T) -> HStack {
        HStack {
            view_base: view::Base::default(),
            spacing: 0.0,
            elements: elements.into_view_group(),
        }
    }

    pub fn spacing(mut self, distance: f64) -> Self {
        self.spacing = distance;
        self
    }
}

impl view::Draw for HStack {
    fn draw(&self, mut cx: view::Context, scene: &mut vello::Scene) {
        println!("L{} HStack {} {}", cx.level, self.size(), cx.origin);
        cx.level += 1;

        // Given that the root view is a container and always drawn,
        // only view containers need to check for element visibility.
        let process =
            |element: &Box<dyn View>, cx: &mut view::Context, scene: &mut vello::Scene| {
                element.draw(
                    view::Context {
                        origin: view::Origin {
                            x: cx.origin.x + self.padding_left(),
                            y: cx.origin.y + self.padding_top(),
                        },
                        ..*cx
                    },
                    scene,
                );
                cx.origin.x += element.width();
                cx.origin.x += element.padding_horizontal();
                cx.origin.x += self.spacing;
            };

        for element in self.elements.iter().filter(|e| e.visible()) {
            if let Some(list) = element.as_any().downcast_ref::<Loop>() {
                for element in &list.elements {
                    process(element, &mut cx, scene);
                }
            } else {
                process(element, &mut cx, scene);
            }
        }
    }
}

#[derive(Default, ViewBase)]
pub struct Rectangle {
    view_base: view::Base,
}

impl view::Draw for Rectangle {
    fn draw(&self, cx: view::Context, scene: &mut vello::Scene) {
        println!("L{} Rectangle {} {}", cx.level, self.size(), cx.origin);

        let rect = vello::kurbo::Rect::new(
            cx.origin.x + self.padding_left(),
            cx.origin.y + self.padding_top(),
            cx.origin.x + self.padding_left() + self.width(),
            cx.origin.y + self.padding_top() + self.height(),
        );

        let rect_stroke_color = Color::rgb(0.5, 0.5, 1.0);
        scene.stroke(
            &vello::kurbo::Stroke::new(2.0),
            vello::kurbo::Affine::IDENTITY,
            rect_stroke_color,
            None,
            &rect,
        );
    }
}

#[derive(Default, ViewBase)]
pub struct Circle {
    view_base: view::Base,
}

impl Circle {
    pub fn diameter(self, diameter: f64) -> Self {
        self.size(diameter, diameter)
    }

    pub fn radius(self, radius: f64) -> Self {
        self.size(radius * 2.0, radius * 2.0)
    }
}

impl view::Draw for Circle {
    fn draw(&self, cx: view::Context, scene: &mut vello::Scene) {
        println!("L{} Circle {} {}", cx.level, self.size(), cx.origin);

        // Draw a filled circle
        let circle = vello::kurbo::Circle::new(
            (
                cx.origin.x + self.padding_left() + self.width() / 2.0,
                cx.origin.y + self.padding_top() + self.width() / 2.0,
            ),
            self.width() / 2.0,
        );

        let circle_fill_color = Color::rgb(1.0, 1.0, 1.0);
        scene.fill(
            vello::peniko::Fill::NonZero,
            vello::kurbo::Affine::IDENTITY,
            circle_fill_color,
            None,
            &circle,
        );
    }
}
