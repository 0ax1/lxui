#![allow(dead_code, unused_parens)]

mod view;
use view::*;

mod shapes;
use shapes::*;

mod rendering;
use rendering::*;

use winit::dpi::LogicalSize;
use winit::event::*;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::*;

use vello::peniko::Color;

fn init_winit_window(event_loop: &ActiveEventLoop) -> std::sync::Arc<winit::window::Window> {
    let attr = winit::window::Window::default_attributes()
        .with_inner_size(LogicalSize::new(750, 180))
        .with_resizable(true)
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

    let mut cx = view::Context {
        origin: view::Origin { x: 0.0, y: 0.0 },
        scale: 1.0,
        level: 0,
        cursor_position: view::CursorPosition { x: 0.0, y: 0.0 },
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
                    cx.cursor_position = view::CursorPosition {
                        x: position.x,
                        y: position.y,
                    };
                    render_state.window.request_redraw();
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
                    view_tree().draw(cx, &mut scene);
                    rendering::render(render_state, &render_cx, &scene, &mut renderers);
                }
                _ => {}
            }
        }
        _ => {}
    });

    println!("{:?}", result);
}

#[rustfmt::skip]
fn view_tree() -> impl view::AnyView {
    VStack::new((
        HStack::new((
            Rectangle::default()
                .size(100.0, 100.0)
                .stroke(Color::rgb8(122, 122, 255), 2.0),

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
            .size(100.0, 100.0),
        ))
        .spacing(40.0)
        .size(430.0, 100.0),

        HStack::new((
            Loop::new(14, |idx|
                Circle::default()
                    .fill(Color::rgb8(255, 255, 255))
                    .diameter(10.0 * (idx + 1) as f64 / 2.0)
                    .visible(idx % 2 == 0)
            )
        ))
        .spacing(20.0)
        .size(830.0, 200.0),
    ))
    .size(620.0, 300.0)
    .spacing(100.0)
    .padding_top(40.0)
    .padding_left(40.0)
}

fn main() {
    init_runloop();
}
