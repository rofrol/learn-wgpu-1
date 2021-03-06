use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::{Window, WindowBuilder},
};

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,

    clear_color: wgpu::Color,

    hidpi_factor: f64,
    size: winit::dpi::LogicalSize,
}

impl State {
    fn new(window: &Window) -> Self {
        let hidpi_factor = window.hidpi_factor();
        let size = window.inner_size();
        let physical_size = size.to_physical(hidpi_factor);

        let surface = wgpu::Surface::create(window);

        let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
            ..Default::default()
        }).unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: Default::default(),
        });

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: physical_size.width.round() as u32,
            height: physical_size.height.round() as u32,
            present_mode: wgpu::PresentMode::Vsync,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let clear_color = wgpu::Color::BLACK;

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,

            clear_color,

            hidpi_factor,
            size,
        }
    }

    fn update_hidpi_and_resize(&mut self, new_hidpi_factor: f64) {
        self.hidpi_factor = new_hidpi_factor;
        self.resize(self.size);
    }

    fn resize(&mut self, new_size: winit::dpi::LogicalSize) {
        let physical_size = new_size.to_physical(self.hidpi_factor);
        self.size = new_size;
        self.sc_desc.width = physical_size.width.round() as u32;
        self.sc_desc.height = physical_size.height.round() as u32;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved {
                position,
                ..
            } => {
                self.clear_color = wgpu::Color {
                    r: position.x as f64 / self.size.width as f64,
                    g: position.y as f64 / self.size.height as f64,
                    b: 1.0,
                    a: 1.0,
                };
                true
            }
            _ => false,
        }
    }

    fn update(&mut self) {

    }

    fn render(&mut self) {
        let frame = self.swap_chain.get_next_texture();

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            todo: 0,
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: self.clear_color,
                    }
                ],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(&[
            encoder.finish()
        ]);
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    let mut state = State::new(&window);
    
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => if state.input(event) {
                *control_flow = ControlFlow::Wait;
            } else { 
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        input,
                        ..
                    } => {
                        match input {
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            _ => *control_flow = ControlFlow::Wait,
                        }
                    }
                    WindowEvent::Resized(logical_size) => {
                        state.resize(*logical_size);
                        *control_flow = ControlFlow::Wait;
                    }
                    WindowEvent::HiDpiFactorChanged(new_hidpi_factor) => {
                        state.update_hidpi_and_resize(*new_hidpi_factor);
                        *control_flow = ControlFlow::Wait;
                    }
                    _ => *control_flow = ControlFlow::Wait,
                }
            }
            Event::EventsCleared => {
                state.update();
                state.render();
                *control_flow = ControlFlow::Wait;
            }
            _ => *control_flow = ControlFlow::Wait,
        }
    });
}