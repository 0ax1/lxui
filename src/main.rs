#![allow(dead_code, unused_parens)]

use macros::AnyView;

mod core;
use core::*;

mod view;
use view::*;

mod rendering;
use rendering::*;

use std::cell::RefCell;
use std::rc::Rc;

use winit::dpi::LogicalSize;
use winit::event::*;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::*;

use vello::peniko::Color;

fn init_winit_window(event_loop: &ActiveEventLoop) -> std::sync::Arc<winit::window::Window> {
    let attr = winit::window::Window::default_attributes()
        .with_inner_size(LogicalSize::new(600, 300))
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
    let view_tree = ViewTree::new();

    let mut cx = core::Context {
        origin: core::Origin { x: 0.0, y: 0.0 },
        scale: 1.0,
        level: 0,
        cursor_position: core::CursorPosition { x: 0.0, y: 0.0 },
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
                    cx.cursor_position = core::CursorPosition {
                        x: position.x,
                        y: position.y,
                    };
                }

                WindowEvent::MouseInput { state, button, .. } => {
                    if button == MouseButton::Left && state == ElementState::Pressed {
                        println!("click");
                    }
                }

                WindowEvent::KeyboardInput { event, .. } => {
                    if event.state == ElementState::Pressed {
                        match event.logical_key.as_ref() {
                            Key::Named(NamedKey::ArrowUp) => {
                                cx.scale += 1.0;
                                render_state.window.request_redraw();
                            }
                            Key::Named(NamedKey::ArrowDown) => {
                                cx.scale -= 1.0;
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
                    view_tree.draw(cx, &mut scene);
                    rendering::render(render_state, &render_cx, &scene, &mut renderers);
                }
                _ => {}
            }
        }
        _ => {}
    });

    println!("{:?}", result);
}

#[derive(AnyView)]
pub struct ViewTree {
    view_base: core::Base,
    state: Rc<RefCell<i32>>,
    tree: VStack,
}

impl Draw for ViewTree {
    fn draw(&self, cx: Context, scene: &mut vello::Scene) {
        self.tree.draw(cx, scene);
    }
}

impl ViewTree {
    pub fn new() -> Self {
        let state = Rc::new(RefCell::new(0));

        ViewTree {
            view_base: core::Base::default(),
            state: state.clone(),
            tree: ViewTree::build(state),
        }
    }

    #[rustfmt::skip]
    fn build(state: Rc<RefCell<i32>>) -> VStack {
        VStack::new((
            HStack::new((
                Rectangle::default()
                    .size(100.0, 100.0)
                    .stroke(Color::rgb8(122, 122, 255), 2.0)
                    .on_click(event::callback(&state, {
                        |state| {
                            println!("clicked {}", state);
                        }
                    })),

                Circle::default()
                    .stroke(Color::rgb8(255, 255, 255), 4.0)
                    .diameter(100.0),

                ZStack::new((
                    Rectangle::default()
                        .size(100.0, 100.0)
                        .fill(Color::rgba8(255, 255, 255, 122))
                        .stroke(Color::rgb8(255, 255, 255), 2.0),

                    Circle::default()
                        .diameter(50.0)
                        .fill(Color::rgb8(122, 122, 255))
                        .padding_top(25.0)
                        .padding_left(25.0),
                ))
            ))
            .spacing(40.0),

            HStack::new((
                Loop::new(0..10, |idx|
                    VStack::new((
                        Loop::new(0..10, |idx2|
                            Circle::default()
                                .stroke(Color::rgba8(122, 122, 255, 50), 2.0)
                                .fill(Color::rgb8(25 * idx2 as u8, 25 * idx2 as u8, 25 * idx2 as u8))
                                .diameter(10.0 * (idx + 1) as f64 / 2.0)
                                .visible(idx % 2 == 0)
                            )
                    ))
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
