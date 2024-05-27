use crate::{view, *};
use macros::*;
use vello::peniko::Color;

#[derive(AnyView)]
pub struct Loop {
    view_base: view::Base,
    elements: Vec<Box<dyn view::AnyView>>,
}

impl Loop {
    pub fn new<F, T>(count: i32, func: F) -> Loop
    where
        T: AnyView,
        F: Fn(i32) -> T,
    {
        Loop {
            view_base: view::Base::default(),
            elements: (0..count)
                .map(|idx| Box::new(func(idx)) as Box<dyn AnyView>)
                .collect(),
        }
    }
}

impl view::Draw for Loop {
    fn draw(&self, _: view::Context, _: &mut vello::Scene) {}
}

pub trait Stack: ViewBase {
    fn elements(&self) -> &[Box<dyn view::AnyView>];

    fn recurse(
        &self,
        mut cx: view::Context,
        scene: &mut vello::Scene,
        operation: impl Fn(&Box<dyn AnyView>, &mut view::Context, &mut vello::Scene),
    ) {
        self.scale(cx.scale);

        for element in self.elements().iter().filter(|e| e.visible()) {
            if let Some(list) = element.as_any().downcast_ref::<Loop>() {
                for element in list.elements.iter().filter(|e| e.visible()) {
                    element.scale(cx.scale);
                    operation(element, &mut cx, scene);
                }
            } else {
                element.scale(cx.scale);
                operation(element, &mut cx, scene);
            }
        }
    }
}

#[derive(AnyView)]
pub struct VStack {
    view_base: view::Base,
    spacing: f64,
    elements: Vec<Box<dyn view::AnyView>>,
}

impl VStack {
    pub fn new<T: view::Container>(elements: T) -> VStack {
        VStack {
            view_base: view::Base::default(),
            elements: elements.into_view_container(),
            spacing: 0.0,
        }
    }

    pub fn spacing(mut self, distance: f64) -> Self {
        self.spacing = distance;
        self
    }
}

impl Stack for VStack {
    fn elements(&self) -> &[Box<dyn view::AnyView>] {
        &self.elements
    }
}

impl view::Draw for VStack {
    fn draw(&self, mut cx: view::Context, scene: &mut vello::Scene) {
        println!("L{} VStack {} {}", cx.level, self.size(), cx.origin);
        cx.level += 1;

        let process =
            |element: &Box<dyn AnyView>, cx: &mut view::Context, scene: &mut vello::Scene| {
                element.draw(
                    view::Context {
                        // Apply the origin offset of the VStack itself.
                        origin: view::Origin {
                            x: cx.origin.x + self.padding_left(),
                            y: cx.origin.y + self.padding_top(),
                        },
                        ..*cx
                    },
                    scene,
                );

                // Offset origin.y for the next element in the VStack.
                cx.origin.y += element.height();
                cx.origin.y += element.padding_vertical();
                cx.origin.y += self.spacing;
            };

        self.recurse(cx, scene, process);
    }
}

#[derive(AnyView)]
pub struct HStack {
    view_base: view::Base,
    spacing: f64,
    elements: Vec<Box<dyn view::AnyView>>,
}

impl HStack {
    pub fn new<T: view::Container>(elements: T) -> HStack {
        HStack {
            view_base: view::Base::default(),
            spacing: 0.0,
            elements: elements.into_view_container(),
        }
    }

    pub fn spacing(mut self, distance: f64) -> Self {
        self.spacing = distance;
        self
    }
}

impl Stack for HStack {
    fn elements(&self) -> &[Box<dyn view::AnyView>] {
        &self.elements
    }
}

impl view::Draw for HStack {
    fn draw(&self, mut cx: view::Context, scene: &mut vello::Scene) {
        println!("L{} HStack {} {}", cx.level, self.size(), cx.origin);
        cx.level += 1;

        // Given that the root view is a container and always drawn,
        // only view containers need to check for element visibility.
        let process =
            |element: &Box<dyn AnyView>, cx: &mut view::Context, scene: &mut vello::Scene| {
                element.draw(
                    view::Context {
                        // Apply the origin offset of the HStack itself.
                        origin: view::Origin {
                            x: cx.origin.x + self.padding_left(),
                            y: cx.origin.y + self.padding_top(),
                        },
                        ..*cx
                    },
                    scene,
                );

                // Offset origin.x for the next element in the HStack.
                cx.origin.x += element.width();
                cx.origin.x += element.padding_horizontal();
                cx.origin.x += self.spacing;
            };

        self.recurse(cx, scene, process);
    }
}

#[derive(AnyView)]
pub struct ZStack {
    view_base: view::Base,
    elements: Vec<Box<dyn view::AnyView>>,
}

impl ZStack {
    pub fn new<T: view::Container>(elements: T) -> ZStack {
        ZStack {
            view_base: view::Base::default(),
            elements: elements.into_view_container(),
        }
    }
}

impl Stack for ZStack {
    fn elements(&self) -> &[Box<dyn view::AnyView>] {
        &self.elements
    }
}

impl view::Draw for ZStack {
    fn draw(&self, mut cx: view::Context, scene: &mut vello::Scene) {
        println!("L{} ZStack {} {}", cx.level, self.size(), cx.origin);
        cx.level += 1;

        let process =
            |element: &Box<dyn AnyView>, cx: &mut view::Context, scene: &mut vello::Scene| {
                element.draw(
                    view::Context {
                        // Apply the origin offset of the ZStack itself.
                        origin: view::Origin {
                            x: cx.origin.x + self.padding_left(),
                            y: cx.origin.y + self.padding_top(),
                        },
                        ..*cx
                    },
                    scene,
                );
            };

        self.recurse(cx, scene, process);
    }
}

#[derive(Default, AnyView)]
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

        let is_hovered = (rect.x0..=rect.x1).contains(&cx.cursor_position.x)
            && (rect.y0..=rect.y1).contains(&cx.cursor_position.y);

        let rect_stroke_color = Color::rgb(0.5, 0.5, 1.0);

        if is_hovered {
            scene.fill(
                vello::peniko::Fill::NonZero,
                vello::kurbo::Affine::IDENTITY,
                rect_stroke_color,
                None,
                &rect,
            );
        }
        scene.stroke(
            &vello::kurbo::Stroke::new(2.0),
            vello::kurbo::Affine::IDENTITY,
            rect_stroke_color,
            None,
            &rect,
        );
    }
}

#[derive(Default, AnyView)]
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
                cx.origin.y + self.padding_top() + self.height() / 2.0,
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
