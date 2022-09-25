use std::collections::HashSet;
use std::path::Path;
use std::time::{Duration, Instant};

use image::io::Reader as ImageReader;
use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use pixels::wgpu::{RequestAdapterOptions, PowerPreference};
use winit::dpi::LogicalSize;
use winit::event::VirtualKeyCode;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};
use winit_input_helper::WinitInputHelper;

pub struct Context {
    pub screen_width: u32,
    pub screen_height: u32,
    pub vsync_enabled: bool,
}

pub struct Engine {
    pub window: Window,
    pub pixel_buffer: Pixels,
    pub resource_manager: ResourceManager,
}

#[derive(Copy, Clone, Debug)]
pub struct ResourceHandle {
    pub id: usize,
    _index: usize,
}

pub struct ImageResource {
    pub bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

pub struct ResourceManager {
    _handle_id: usize,
    _handles: HashSet<usize>, // can probably be a vec and do a binary search since it will always be sorted..?
    _images: Vec<Option<ImageResource>>,
    _available_image_indexes: Vec<usize>,
}

impl ResourceManager {
    fn new() -> Self {
        Self {
            _handle_id: 0,
            _handles: HashSet::new(),
            _images: Vec::new(),
            _available_image_indexes: Vec::new(),
        }
    }
    fn create_image_handle(&mut self) -> ResourceHandle {
        let handle_id = self._handle_id;
        self._handle_id += 1;
        self._handles.insert(handle_id);
        let index = match self._available_image_indexes.pop() {
            Some(i) => i,
            None => self._images.len()
        };
        ResourceHandle {
            id: handle_id,
            _index: index,
        }
    }
    ///load an image and create a new handle to store it with
    pub fn load_image(&mut self, path: &Path) -> ResourceHandle {
        let image = ImageReader::open(path)
            .unwrap()
            .decode()
            .unwrap()
            .to_rgba8();
        let width = image.width();
        let height = image.height();
        let image = ImageResource {
            bytes: image.into_vec(),
            width,
            height,
        };
        let handle = self.create_image_handle();
        if handle._index < self._images.len() {
            self._images[handle._index] = Some(image);
        } else {
            self._images.push(Some(image));
        }
        handle
    }
    pub fn get_image(&mut self, handle: ResourceHandle) -> Option<&ImageResource>{
        if self._handles.contains(&handle.id) {
            return Some(self._images[handle._index].as_ref().unwrap());
        }
        None
    }
    pub fn delete_image(&mut self, handle: ResourceHandle) {
        if self._handles.remove(&handle.id) {
            self._images[handle._index] = None;
            self._available_image_indexes.push(handle._index);
        }
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
    let mut input = WinitInputHelper::new();
    let ctx = game_state.context();
    let window = {
        let size = LogicalSize::new(ctx.screen_width as f64, ctx.screen_height as f64);
        WindowBuilder::new()
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .expect("Error constructing window")
    };
    let pixels = {
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
    let mut engine = Engine {
        window,
        pixel_buffer: pixels,
        resource_manager: ResourceManager::new(),
    };
    let mut t1 = Instant::now();
    if !game_state.on_create(&mut engine) {
        game_state.on_exit();
        return;
    }
    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                game_state.on_exit();
                return;
            }
            if let Some(size) = input.window_resized() {
                engine.pixel_buffer.resize_surface(size.width, size.height);
            }
            let elapsed_time = t1.elapsed();
            t1 = Instant::now();
            if !game_state.on_update(elapsed_time, &mut engine) {
                game_state.on_exit();
                return
            }
            engine.pixel_buffer.render().unwrap();
            engine.window.request_redraw();
        }
    });
}
