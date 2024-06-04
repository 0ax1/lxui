use crate::{core, *};
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

    // Given that the root view is a container and always drawn,
    // only view containers need to check for element visibility.
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

impl core::Layout for VStack {
    fn layout(&self, mut cx: core::Context) {
        let mut width = 0.0;
        let mut height = 0.0;
        let mut count = 0.0;

        self.view_base.origin.set(vello::kurbo::Point {
            x: cx.location.x + self.padding_left(),
            y: cx.location.y + self.padding_top(),
        });

        let process = |element: &Box<dyn AnyView>| {
            element.layout(core::Context {
                // Apply the origin offset of the VStack itself.
                location: kurbo::Point {
                    x: cx.location.x + self.padding_left(),
                    y: cx.location.y + self.padding_top(),
                },
                ..cx
            });

            // Offset origin.y for the next element in the VStack.
            let spacing = self.spacing * core::ui_scale();
            cx.location.y += element.height() + element.padding_vertical() + spacing;
            height += element.height() + element.padding_vertical();
            width = f64::max(width, element.width() + element.padding_horizontal());
            count += 1.0;
        };

        self.recurse_stack(process);

        if self.width() == 0.0 {
            self.view_base.size.set(vello::kurbo::Size {
                width: width / core::ui_scale(),
                height: self.view_base.size.get().height,
            });
        }

        if self.height() == 0.0 {
            let spacing = f64::max(count - 1.0, 0.0) * self.spacing * core::ui_scale();
            self.view_base.size.set(vello::kurbo::Size {
                width: self.view_base.size.get().width,
                height: (height + spacing) / core::ui_scale(),
            });
        }
    }
}

impl core::Draw for VStack {
    fn draw(&self, cx: core::Context, scene: &mut vello::Scene) {
        println!(
            "L{} VStack size: {} origin: {}",
            cx.level,
            self.size(),
            self.origin()
        );

        self.recurse_stack(|element: &Box<dyn AnyView>| {
            element.draw(
                core::Context {
                    level: cx.level + 1,
                    ..cx
                },
                scene,
            );
        });
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

impl core::Layout for HStack {
    fn layout(&self, mut cx: core::Context) {
        let mut width = 0.0;
        let mut height = 0.0;
        let mut count = 0.0;

        self.view_base.origin.set(vello::kurbo::Point {
            x: cx.location.x + self.padding_left(),
            y: cx.location.y + self.padding_top(),
        });

        let process = |element: &Box<dyn AnyView>| {
            element.layout(core::Context {
                // Apply the origin offset of the HStack itself.
                location: kurbo::Point {
                    x: cx.location.x + self.padding_left(),
                    y: cx.location.y + self.padding_top(),
                },
                ..cx
            });

            // Offset origin.x for the next element in the HStack.
            let spacing = self.spacing * core::ui_scale();
            cx.location.x += element.width() + element.padding_horizontal() + spacing;
            width += element.width() + element.padding_horizontal();
            height = f64::max(height, element.height() + element.padding_vertical());
            count += 1.0;
        };

        self.recurse_stack(process);

        if self.width() == 0.0 {
            let spacing = f64::max(count - 1.0, 0.0) * self.spacing * core::ui_scale();
            self.view_base.size.set(vello::kurbo::Size {
                width: (width + spacing) / core::ui_scale(),
                height: self.view_base.size.get().height,
            });
        }

        if self.height() == 0.0 {
            self.view_base.size.set(vello::kurbo::Size {
                width: self.view_base.size.get().width,
                height: height / core::ui_scale(),
            });
        }
    }
}

impl core::Draw for HStack {
    fn draw(&self, cx: core::Context, scene: &mut vello::Scene) {
        println!(
            "L{} HStack size: {} origin: {}",
            cx.level,
            self.size(),
            self.origin()
        );

        self.recurse_stack(|element: &Box<dyn AnyView>| {
            element.draw(
                core::Context {
                    level: cx.level + 1,
                    ..cx
                },
                scene,
            );
        });
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

impl core::Layout for ZStack {
    fn layout(&self, cx: Context) {
        let mut width = 0.0;
        let mut height = 0.0;

        self.view_base.origin.set(vello::kurbo::Point {
            x: cx.location.x + self.padding_left(),
            y: cx.location.y + self.padding_top(),
        });

        let process = |element: &Box<dyn AnyView>| {
            element.layout(core::Context {
                // Apply the origin offset of the ZStack itself.
                location: kurbo::Point {
                    x: cx.location.x + self.padding_left(),
                    y: cx.location.y + self.padding_top(),
                },
                ..cx
            });

            width = f64::max(width, element.width() + element.padding_horizontal());
            height = f64::max(height, element.height() + element.padding_vertical());
        };

        self.recurse_stack(process);

        if self.width() == 0.0 {
            self.view_base.size.set(vello::kurbo::Size {
                width: width / core::ui_scale(),
                height: self.view_base.size.get().width,
            });
        }

        if self.height() == 0.0 {
            self.view_base.size.set(vello::kurbo::Size {
                width: self.view_base.size.get().width,
                height: height / core::ui_scale(),
            });
        }
    }
}

impl core::Draw for ZStack {
    fn draw(&self, cx: core::Context, scene: &mut vello::Scene) {
        println!(
            "L{} ZStack size: {} origin: {}",
            cx.level,
            self.size(),
            self.origin()
        );

        self.recurse_stack(|element: &Box<dyn AnyView>| {
            element.draw(
                core::Context {
                    level: cx.level + 1,
                    ..cx
                },
                scene,
            );
        });
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
        println!("L{} Rectangle {} {}", cx.level, self.size(), self.origin());

        let rect = self.rect();

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
        println!(
            "L{} Circle size: {} origin: {}",
            cx.level,
            self.size(),
            self.origin()
        );

        let rect = self.rect();
        let circle = vello::kurbo::Circle::new(
            (rect.x0 + self.width() / 2.0, rect.y0 + self.height() / 2.0),
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
