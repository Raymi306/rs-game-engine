use std::borrow::Borrow;
use std::time::{Duration, Instant};

use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use pixels::wgpu::{RequestAdapterOptions, PowerPreference};
use winit::dpi::LogicalSize;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

use resource::{ResourceManager, ImageResourceSlice};
use crate::resource::ImageResource;

pub mod constants;
pub mod drawing;
pub mod resource;
pub mod timer;
pub mod types;

#[derive(Clone, Debug)]
pub struct Context {
    pub screen_width: u32,
    pub screen_height: u32,
    pub vsync_enabled: bool,
}

pub struct Screen {
    pixels: Pixels,
    screen_width: u32,
    screen_height: u32,
}

impl ImageResource for Screen {
    fn width(&self) -> u32 {
        self.screen_width
    }

    fn height(&self) -> u32 {
        self.screen_height
    }

    fn get_buf(&self) -> &[u8] {
        unimplemented!("Pixels doesn't provide a read only view")
    }

    fn get_buf_mut(&mut self) -> &mut [u8] {
        self.pixels.get_frame()
    }
}

pub struct Engine {
    pub screen: Screen,
    pub window: Window,
    pub resource_manager: ResourceManager,
    pub input: WinitInputHelper,
}

impl Engine {
    pub fn resize_buffer(&mut self, width: u32, height: u32) {
        self.screen.screen_width = width;
        self.screen.screen_height = height;
        self.screen.pixels.resize_buffer(width, height);
    }
    pub fn resize_surface(&mut self, width: u32, height: u32) {
        self.screen.pixels.resize_surface(width, height);
    }
    pub fn render(&mut self) {
        self.screen.pixels.render().unwrap();
    }
}

pub trait GameState {
    fn on_create(&mut self, _engine: &mut Engine) -> bool { true }
    fn on_update(&mut self, _elapsed_time: Duration, _engine: &mut Engine) -> bool { true }
    fn on_exit(&mut self) {}
    fn context(&self) -> &Context;
}

pub fn run<T: GameState + 'static>(mut game_state: T) {
    let event_loop = EventLoop::new();
    let input = WinitInputHelper::new();
    let ctx = game_state.context();
    let window = {
        let size = LogicalSize::new(ctx.screen_width as f64, ctx.screen_height as f64);
        WindowBuilder::new()
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .expect("Error constructing window")
    };
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        PixelsBuilder::new(ctx.screen_width, ctx.screen_height, surface_texture)
            .request_adapter_options(RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .enable_vsync(ctx.vsync_enabled)
            .build()
            .expect("Error constructing pixel buffer")
    };
    let resource_manager = ResourceManager::new();
    let mut engine = Engine {
        screen: Screen {
            pixels: pixels,
            screen_width: ctx.screen_width,
            screen_height: ctx.screen_height,
        },
        window,
        resource_manager,
        input,
    };
    let mut t1 = Instant::now();
    if !game_state.on_create(&mut engine) {
        game_state.on_exit();
        return;
    }
    event_loop.run(move |event, _, control_flow| {
        if engine.input.update(&event) {
            if engine.input.quit() {
                *control_flow = ControlFlow::Exit;
                game_state.on_exit();
                return;
            }
            if let Some(size) = engine.input.window_resized() {
                engine.resize_surface(size.width, size.height);
            }
            let elapsed_time = t1.elapsed();
            t1 = Instant::now();
            if !game_state.on_update(elapsed_time, &mut engine) {
                game_state.on_exit();
                return;
            }
            engine.render();
            engine.window.request_redraw();
        }
    });
}
