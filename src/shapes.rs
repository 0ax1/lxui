use crate::{view, *};
use macros::*;
use vello::{kurbo, peniko};

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
    fn view_sequence(&self) -> &[Box<dyn view::AnyView>];

    fn recurse_stack(
        &self,
        mut cx: view::Context,
        scene: &mut vello::Scene,
        operation: impl Fn(&Box<dyn AnyView>, &mut view::Context, &mut vello::Scene),
    ) {
        for element in self.view_sequence().iter().filter(|e| e.visible()) {
            if let Some(list) = element.as_any().downcast_ref::<Loop>() {
                for element in list.elements.iter().filter(|e| e.visible()) {
                    operation(element, &mut cx, scene);
                }
            } else {
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
    pub fn new<T: view::ViewSequence>(elements: T) -> VStack {
        VStack {
            view_base: view::Base::default(),
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
    fn view_sequence(&self) -> &[Box<dyn view::AnyView>] {
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

        self.recurse_stack(cx, scene, process);
    }
}

#[derive(AnyView)]
pub struct HStack {
    view_base: view::Base,
    spacing: f64,
    elements: Vec<Box<dyn view::AnyView>>,
}

impl HStack {
    pub fn new<T: view::ViewSequence>(elements: T) -> HStack {
        HStack {
            view_base: view::Base::default(),
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
    fn view_sequence(&self) -> &[Box<dyn view::AnyView>] {
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

        self.recurse_stack(cx, scene, process);
    }
}

#[derive(AnyView)]
pub struct ZStack {
    view_base: view::Base,
    elements: Vec<Box<dyn view::AnyView>>,
}

impl ZStack {
    pub fn new<T: view::ViewSequence>(elements: T) -> ZStack {
        ZStack {
            view_base: view::Base::default(),
            elements: elements.into_view_sequence(),
        }
    }
}

impl Stack for ZStack {
    fn view_sequence(&self) -> &[Box<dyn view::AnyView>] {
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

        self.recurse_stack(cx, scene, process);
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
    view_base: view::Base,
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

impl view::Draw for Rectangle {
    fn draw(&self, cx: view::Context, scene: &mut vello::Scene) {
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
    view_base: view::Base,
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

impl view::Draw for Circle {
    fn draw(&self, cx: view::Context, scene: &mut vello::Scene) {
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
