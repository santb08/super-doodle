use winit::window::Window;

pub struct ViewportDesc {
    pub window: Window,
    pub background: wgpu::Color,
    pub surface: wgpu::Surface,
}

pub struct Viewport {
    pub desc: ViewportDesc,
    pub config: wgpu::SurfaceConfiguration,
}

impl ViewportDesc {
    pub fn new(window: Window, background: wgpu::Color, instance: &wgpu::Instance) -> Self {
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        Self {
            window,
            background,
            surface,
        }
    }

    pub fn build(self, adapter: &wgpu::Adapter, device: &wgpu::Device) -> Viewport {
        let size = self.window.inner_size();

        let caps = self.surface.get_capabilities(adapter);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: caps.formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };

        self.surface.configure(device, &config);

        Viewport { desc: self, config }
    }
}

impl Viewport {
    pub fn resize(&mut self, device: &wgpu::Device, size: winit::dpi::PhysicalSize<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;
        self.desc.surface.configure(device, &self.config);
    }
    pub fn get_current_texture(&mut self) -> wgpu::SurfaceTexture {
        self.desc
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture")
    }
}
