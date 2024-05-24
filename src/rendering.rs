use vello::peniko::Color;
use vello::{Renderer, RendererOptions};

pub struct ActiveRenderState<'s> {
    pub surface: vello::util::RenderSurface<'s>,
    pub window: std::sync::Arc<winit::window::Window>,
}

pub enum RenderState<'s> {
    Active(ActiveRenderState<'s>),
    Suspended(Option<std::sync::Arc<winit::window::Window>>),
}

pub fn init_renderer(
    render_cx: &vello::util::RenderContext,
    surface: &vello::util::RenderSurface,
) -> Renderer {
    Renderer::new(
        &render_cx.devices[surface.dev_id].device,
        RendererOptions {
            surface_format: Some(surface.format),
            use_cpu: false,
            antialiasing_support: vello::AaSupport::all(),
            num_init_threads: std::num::NonZeroUsize::new(1),
        },
    )
    .expect("error: initializing vello renderer")
}

pub fn render(
    render_state: &ActiveRenderState,
    render_cx: &vello::util::RenderContext,
    scene: &vello::Scene,
    renderers: &mut Vec<Option<Renderer>>,
) {
    let surface = &render_state.surface;
    let width = surface.config.width;
    let height = surface.config.height;
    let device_handle = &render_cx.devices[surface.dev_id];
    let surface_texture = surface
        .surface
        .get_current_texture()
        .expect("error: getting surface texture");

    renderers[surface.dev_id]
        .as_mut()
        .unwrap()
        .render_to_surface(
            &device_handle.device,
            &device_handle.queue,
            &scene,
            &surface_texture,
            &vello::RenderParams {
                base_color: Color::BLACK,
                width,
                height,
                antialiasing_method: vello::AaConfig::Msaa16,
            },
        )
        .expect("error: rendering to surface");

    surface_texture.present();
    device_handle.device.poll(wgpu::Maintain::Poll);
}
