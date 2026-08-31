use crate::event_bridge::EventBridge;
use crate::window::WindowOptions;
use quick_core::event::Event;
use quick_core::geometry::Size;
use quick_render::canvas::Canvas;
use quick_render::rasterizer::SoftwareRasterizer;
use std::num::NonZeroU32;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

pub trait AppController {
    fn render_frame(&mut self, size: Size) -> &Canvas;
    fn handle_event(&mut self, event: &Event, size: Size) -> bool;
}

pub struct WindowRunner<C: AppController> {
    options: WindowOptions,
    controller: C,
    bridge: EventBridge,
    window: Option<Arc<Window>>,
    surface: Option<softbuffer::Surface<Arc<Window>, Arc<Window>>>,
    current_size: Size,
    scale_factor: f32,
}

impl<C: AppController> WindowRunner<C> {
    pub fn new(options: WindowOptions, controller: C) -> Self {
        let size = options.size;
        Self {
            options,
            controller,
            bridge: EventBridge::new(),
            window: None,
            surface: None,
            current_size: size,
            scale_factor: 1.0,
        }
    }

    pub fn scale_factor(&self) -> f32 {
        self.scale_factor
    }

    pub fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Wait);
        event_loop.run_app(&mut self)?;
        Ok(())
    }
}

impl<C: AppController> ApplicationHandler for WindowRunner<C> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let attributes = Window::default_attributes()
            .with_title(&self.options.title)
            .with_inner_size(LogicalSize::new(
                self.options.size.width as f64,
                self.options.size.height as f64,
            ))
            .with_resizable(self.options.resizable);

        match event_loop.create_window(attributes) {
            Ok(w) => {
                let window_arc = Arc::new(w);
                self.scale_factor = window_arc.scale_factor() as f32;
                self.window = Some(window_arc.clone());

                match softbuffer::Context::new(window_arc.clone()) {
                    Ok(context) => match softbuffer::Surface::new(&context, window_arc.clone()) {
                        Ok(surface) => {
                            self.surface = Some(surface);
                            window_arc.request_redraw();
                        }
                        Err(err) => {
                            eprintln!("Failed to create softbuffer surface: {:?}", err);
                        }
                    },
                    Err(err) => {
                        eprintln!("Failed to create softbuffer context: {:?}", err);
                    }
                }
            }
            Err(err) => {
                eprintln!("Failed to create Wayland/X11 window: {:?}", err);
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.scale_factor = scale_factor as f32;
                if let Some(ref window) = self.window {
                    let phys = window.inner_size();
                    let sf = if self.scale_factor > 0.0 { self.scale_factor } else { 1.0 };
                    self.current_size = Size::new(phys.width as f32 / sf, phys.height as f32 / sf);
                    window.request_redraw();
                }
            }
            WindowEvent::Resized(physical_size) => {
                let sf = if self.scale_factor > 0.0 { self.scale_factor } else { 1.0 };
                let w = physical_size.width as f32 / sf;
                let h = physical_size.height as f32 / sf;
                self.current_size = Size::new(w, h);

                if let (Some(surface), Some(w_nz), Some(h_nz)) = (
                    &mut self.surface,
                    NonZeroU32::new(physical_size.width),
                    NonZeroU32::new(physical_size.height),
                ) {
                    let _ = surface.resize(w_nz, h_nz);
                }

                if let Some(ref window) = self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                let sf = if self.scale_factor > 0.0 { self.scale_factor } else { 1.0 };
                let width = (self.current_size.width * sf).round() as u32;
                let height = (self.current_size.height * sf).round() as u32;

                let canvas = self.controller.render_frame(self.current_size);

                if let Some(ref mut surface) = self.surface {
                    if let (Some(w_nz), Some(h_nz)) = (NonZeroU32::new(width), NonZeroU32::new(height)) {
                        let _ = surface.resize(w_nz, h_nz);
                    }

                    if let Ok(mut buffer) = surface.buffer_mut() {
                        SoftwareRasterizer::render_to_buffer(canvas, width, height, &mut buffer);
                        let _ = buffer.present();
                    }
                }
            }
            other_event => {
                if let Some(quick_event) = self.bridge.translate_event_scaled(&other_event, self.scale_factor) {
                    let handled = self.controller.handle_event(&quick_event, self.current_size);
                    if handled {
                        if let Some(ref window) = self.window {
                            window.request_redraw();
                        }
                    }
                }
            }
        }
    }
}
