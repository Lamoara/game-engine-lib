use std::{hash::DefaultHasher, sync::Arc};

use wgpu::RenderPipeline;

pub struct State {
    pub window: Arc<winit::window::Window>,

    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,

    is_surface_configured: bool,
    
    render_pipelines: RenderPipelines,
}

struct RenderPipelines {
    shadow_passes: wgpu::RenderPipeline,
    skybox_pass: wgpu::RenderPipeline,
    opaque_pass: wgpu::RenderPipeline,
    transparent_pass: wgpu::RenderPipeline,
    ui_pass: wgpu::RenderPipeline,
}


impl State {
    pub async fn new(window: Arc<winit::window::Window>, display_handle: winit::event_loop::OwnedDisplayHandle) -> anyhow::Result<State> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            flags: wgpu::InstanceFlags::default(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
            backend_options: wgpu::BackendOptions::from_env_or_default(),
            display: Some(Box::new(display_handle)),
            
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }).await.unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::defaults(),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            memory_hints: Default::default(),
            trace: wgpu::Trace::Off,
        })
        .await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: surface_caps.alpha_mode[0],
            view_formats: vec![],
        };


        Ok(Self{
            window,
            device: device,
            queue: queue,
            surface: surface,
            render_pipelines: todo!(),
        })
    }
}
