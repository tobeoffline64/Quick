#[cfg(feature = "vello")]
use std::sync::Arc;
#[cfg(feature = "vello")]
use vello::wgpu;
#[cfg(feature = "vello")]
use vello::{Renderer, RendererOptions, Scene};
#[cfg(feature = "vello")]
use winit::window::Window;

#[cfg(feature = "vello")]
pub struct VelloSurface {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
    pub renderer: Renderer,
    pub scene: Scene,
}

#[cfg(feature = "vello")]
impl VelloSurface {
    pub fn new(window: Arc<Window>, width: u32, height: u32) -> Result<Self, Box<dyn std::error::Error>> {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone())?;

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .ok_or("Failed to find compatible WGPU adapter for Vello")?;

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Quick Vello Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::default(),
            },
            None,
        ))?;

        let device = Arc::new(device);
        let queue = Arc::new(queue);

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| matches!(f, wgpu::TextureFormat::Rgba8Unorm | wgpu::TextureFormat::Bgra8Unorm))
            .unwrap_or(wgpu::TextureFormat::Bgra8Unorm);

        let alpha_mode = surface_caps
            .alpha_modes
            .iter()
            .copied()
            .find(|a| matches!(a, wgpu::CompositeAlphaMode::Opaque | wgpu::CompositeAlphaMode::Auto))
            .unwrap_or(surface_caps.alpha_modes[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: width.max(1),
            height: height.max(1),
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let renderer = Renderer::new(
            &device,
            RendererOptions {
                surface_format: Some(surface_format),
                use_cpu: false,
                antialiasing_support: vello::AaSupport::all(),
                num_init_threads: None,
            },
        )?;

        Ok(Self {
            device,
            queue,
            surface,
            config,
            renderer,
            scene: Scene::new(),
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self, width: u32, height: u32) -> Result<(), Box<dyn std::error::Error>> {
        let surface_texture = self.surface.get_current_texture()?;

        self.renderer.render_to_surface(
            &self.device,
            &self.queue,
            &self.scene,
            &surface_texture,
            &vello::RenderParams {
                base_color: vello::peniko::Color::BLACK,
                width,
                height,
                antialiasing_method: vello::AaConfig::Msaa16,
            },
        )?;

        surface_texture.present();
        Ok(())
    }
}
