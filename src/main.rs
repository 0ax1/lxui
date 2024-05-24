#![allow(dead_code)]

mod view;
use view::*;

mod shapes;
use shapes::*;

mod rendering;
use rendering::*;

use winit::dpi::LogicalSize;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};

fn init_winit_window(event_loop: &ActiveEventLoop) -> std::sync::Arc<winit::window::Window> {
    let attr = winit::window::Window::default_attributes()
        .with_inner_size(LogicalSize::new(750, 180))
        .with_resizable(true)
        .with_title("fors: gpu go brr");

    std::sync::Arc::new(event_loop.create_window(attr).expect("error: creating window"))
}


fn init_runloop() {
    let mut render_cx = vello::util::RenderContext::new().expect("error: creating render context");
    let mut renderers: Vec<Option<vello::Renderer>> = [].into();
    let mut render_state = RenderState::Suspended(None);
    let mut scene = vello::Scene::new();
    let event_loop = EventLoop::new().expect("error: creating runloop");

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
            renderers[surface.dev_id]
                .get_or_insert_with(|| init_renderer(&render_cx, &surface));

            render_state = RenderState::Active(ActiveRenderState { window, surface });
            event_loop.set_control_flow(ControlFlow::Poll);
        }

        winit::event::Event::Suspended => {
            if let RenderState::Active(state) = &render_state {
                render_state = RenderState::Suspended(Some(state.window.clone()));
            }
            event_loop.set_control_flow(ControlFlow::Wait);
        }

        winit::event::Event::WindowEvent { event, window_id } => {
            let render_state = match &mut render_state {
                RenderState::Active(state) if state.window.id() == window_id => state,
                _ => return,
            };

            match event {
                winit::event::WindowEvent::CloseRequested => event_loop.exit(),

                winit::event::WindowEvent::Resized(size) => {
                    render_cx.resize_surface(&mut render_state.surface, size.width, size.height);
                    render_state.window.request_redraw();
                }

                winit::event::WindowEvent::RedrawRequested => {
                    scene.reset();
                    view_tree().draw(
                        view::Context {
                            origin: view::Origin { x: 0.0, y: 0.0 },
                            level: 0,
                        },
                        &mut scene,
                    );
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
fn view_tree() -> impl view::View {
    VStack::new((
        HStack::new((
            Rectangle::default()
                .size(100.0, 100.0),

            Circle::default()
                .diameter(100.0)
                .padding_left(40.0),

            Rectangle::default()
                .size(100.0, 100.0)
                .padding_left(40.0),

            Circle::default()
                .diameter(100.0)
                .padding_left(40.0)
        ))
        .size(430.0, 100.0),

        HStack::new((
            Loop::new(20, |idx|
                Circle::default()
                    .diameter(10.0 * (idx + 1) as f64 / 2.0)
            ),
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
