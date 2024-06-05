#![allow(dead_code, unused_parens)]

use macros::AnyView;

mod core;
use core::*;

mod view;
use view::*;

mod rendering;
use rendering::*;

use winit::dpi::LogicalSize;
use winit::event::*;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::*;

use vello::kurbo;
use vello::peniko::Color;

fn init_winit_window(event_loop: &ActiveEventLoop) -> std::sync::Arc<winit::window::Window> {
    let attr = winit::window::Window::default_attributes()
        .with_inner_size(LogicalSize::new(600, 600))
        .with_resizable(true)
        .with_active(true)
        .with_title("fors: gpu go brr");

    std::sync::Arc::new(
        event_loop
            .create_window(attr)
            .expect("error: creating window"),
    )
}

fn init_runloop() {
    let mut render_cx = vello::util::RenderContext::new().expect("error: creating render context");
    let mut renderers: Vec<Option<vello::Renderer>> = [].into();
    let mut render_state = RenderState::Suspended(None);
    let mut scene = vello::Scene::new();
    let event_loop = EventLoop::new().expect("error: creating runloop");
    let mut view_tree = ViewTree::new();

    let mut cx = core::Context {
        location: kurbo::Point { x: 0.0, y: 0.0 },
        level: 0,
    };

    #[allow(deprecated)]
    let result = event_loop.run(move |event, event_loop| match event {
        winit::event::Event::Resumed => {
            let RenderState::Suspended(cached_window) = &mut render_state else {
                return;
            };

            let window = cached_window
                .take()
                .unwrap_or_else(|| init_winit_window(event_loop));

            let size = window.inner_size();
            let surface_future = render_cx.create_surface(
                window.clone(),
                size.width,
                size.height,
                wgpu::PresentMode::AutoVsync,
            );
            let surface = pollster::block_on(surface_future).expect("error: creating surface");

            renderers.resize_with(render_cx.devices.len(), || None);
            renderers[surface.dev_id].get_or_insert_with(|| init_renderer(&render_cx, &surface));

            render_state = RenderState::Active(ActiveRenderState { window, surface });
            event_loop.set_control_flow(ControlFlow::Poll);
        }

        Event::Suspended => {
            if let RenderState::Active(state) = &render_state {
                render_state = RenderState::Suspended(Some(state.window.clone()));
            }
            event_loop.set_control_flow(ControlFlow::Wait);
        }

        Event::WindowEvent { event, window_id } => {
            let render_state = match &mut render_state {
                RenderState::Active(state) if state.window.id() == window_id => state,
                _ => return,
            };

            match event {
                WindowEvent::CloseRequested => event_loop.exit(),

                WindowEvent::CursorMoved { position, .. } => {
                    cx.location = kurbo::Point {
                        x: position.x,
                        y: position.y,
                    };
                }

                WindowEvent::MouseInput { state, button, .. } => {
                    if button == MouseButton::Left && state == ElementState::Pressed {
                        view_tree.root.mouse_down(cx);
                        render_state.window.request_redraw();
                    }
                }

                WindowEvent::KeyboardInput { event, .. } => {
                    if event.state == ElementState::Pressed {
                        match event.logical_key.as_ref() {
                            Key::Named(NamedKey::ArrowUp) => {
                                render_state.window.request_redraw();
                            }
                            Key::Named(NamedKey::ArrowDown) => {
                                render_state.window.request_redraw();
                            }
                            _ => {}
                        }
                    }
                }
                WindowEvent::Resized(size) => {
                    render_cx.resize_surface(&mut render_state.surface, size.width, size.height);
                    render_state.window.request_redraw();
                }

                WindowEvent::RedrawRequested => {
                    scene.reset();
                    cx.location = kurbo::Point::default();
                    view_tree.root = view_tree.rebuild();
                    view_tree.root.layout(cx);
                    view_tree.root.draw(cx, &mut scene);
                    rendering::render(render_state, &render_cx, &scene, &mut renderers);
                }
                _ => {}
            }
        }
        _ => {}
    });

    println!("{:?}", result);
}

pub struct ViewTree {
    view_base: core::Base,
    state: State<i32>,
    root: VStack,
}

impl ViewTree {
    pub fn new() -> Self {
        let state = State::new(0);

        ViewTree {
            view_base: core::Base::default(),
            state: state.clone(),
            root: ViewTree::build(state),
        }
    }

    pub fn rebuild(&self) -> VStack {
        let root = ViewTree::build(self.state.clone());
        // TODO: call rebuild on children
        root
    }

    #[rustfmt::skip]
    fn build(state: State<i32>) -> VStack {
        VStack::new((
            HStack::new((
                Rectangle::default()
                    .size(100.0, 100.0)
                    .stroke(Color::rgb8(122, 122, 122), 2.0 * state.val() as f64)
                    .on_click(core::callback(&state, {
                        |state| {
                            *state += 1;
                            println!("clicked {}", state);
                        }
                    })),

                Circle::default()
                    .stroke(Color::rgb8(255, 255, 255), 4.0)
                    .diameter(100.0)
                    .on_click(core::callback(&state, {
                        |state| {
                            *state += 1;
                            println!("clicked {}", state);
                        }
                    })),

                ZStack::new((
                    Rectangle::default()
                        .size(100.0, 100.0)
                        .fill(Color::rgba8(255, 255, 255, 122))
                        .stroke(Color::rgb8(255, 255, 255), 2.0),

                    Circle::default()
                        .diameter(50.0)
                        .fill(Color::rgb8(122, 122, 255))
                        .padding_top(25.0)
                        .padding_left(25.0)
                        .on_click(core::callback(&state, {
                            |state| {
                                println!("clicked {}", state);
                            }
                        }))
                ))
            ))
            .spacing(40.0),

            HStack::new((
                Loop::new(0..18, |idx|
                    VStack::new((
                        Loop::new(0..10, |idx2|
                            Circle::default()
                                .stroke(Color::rgba8(122, 122, 255, 50), 2.0)
                                .fill(Color::rgb8(25 * idx2 as u8, 25 * idx2 as u8, 25 * idx2 as u8))
                                .diameter(5.0 * (idx + 1) as f64 / 2.0)
                    )))
                    .visible(idx % 2 == 0)
                    .spacing(20.0)
                )
            ))
            .spacing(20.0)
        ))
        .spacing(100.0)
        .padding_top(40.0)
        .padding_left(40.0)
    }
}

fn main() {
    init_runloop();
}
