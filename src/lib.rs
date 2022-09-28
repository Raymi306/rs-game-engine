use std::time::{Duration, Instant};

use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use pixels::wgpu::{RequestAdapterOptions, PowerPreference};
use winit::dpi::LogicalSize;
use winit::event_loop::{EventLoop, ControlFlow};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

use resource::{ResourceManager, ImageResourceSlice};

pub mod constants;
pub mod drawing;
pub mod resource;
pub mod timer;
pub mod types;

pub struct Context {
    pub screen_width: u32,
    pub screen_height: u32,
    pub vsync_enabled: bool,
}

pub struct Engine<'a> {
    _pixels: Pixels,
    screen: ImageResourceSlice<'a>,
    pub window: Window,
    pub resource_manager: ResourceManager,
    pub input: WinitInputHelper,
}

impl Engine<'_> {
    pub fn resize_buffer(&mut self, width: u32, height: u32) {
        self._pixels.resize_buffer(width, height);
        self.screen.set_width(width);
        self.screen.set_height(height);
    }
    pub fn resize_surface(&mut self, width: u32, height: u32) {
        self._pixels.resize_surface(width, height);
    }
    pub fn render(&mut self) {
        self._pixels.render().unwrap();
    }
}

pub trait GameState {
    fn on_create(&mut self, _engine: &mut Engine) -> bool {true}
    fn on_update(&mut self, _elapsed_time: Duration, _engine: &mut Engine) -> bool {true}
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
                compatible_surface: None
            })
            .enable_vsync(ctx.vsync_enabled)
            .build()
            .expect("Error constructing pixel buffer")
    };
    let resource_manager = ResourceManager::new();
    let mut engine = Engine {
        _pixels: pixels,
        screen: ImageResourceSlice::new(ctx.screen_width, ctx.screen_height, pixels.get_frame()),
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
