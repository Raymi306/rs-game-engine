use std::path::Path;
use std::time::Duration;

use engine::{
    run,
    Context,
    Engine,
    GameState,
    ResourceHandle
};

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;

pub struct Demo {
    ctx: Context,
    image_handle_1: Option<ResourceHandle>,
    image_handle_2: Option<ResourceHandle>,
}

impl Demo {
    pub fn new() -> Self {
        let ctx = Context {
            screen_width: SCREEN_WIDTH,
            screen_height: SCREEN_HEIGHT,
            vsync_enabled: false,
        };
        Self {
            ctx,
            image_handle_1: None,
            image_handle_2: None,
        }
    }
}

impl GameState for Demo {
    fn on_create(&mut self, engine: &mut Engine) -> bool {
        self.image_handle_1 = Some(
            engine
            .resource_manager
            .load_image(Path::new("resources/images/test_pattern_1.bmp"))
        );
        self.image_handle_2 = Some(
            engine
            .resource_manager
            .load_image(Path::new("resources/images/test_pattern_2.bmp"))
        );
        true
    }
    fn on_update(&mut self, _elapsed_time: Duration, engine: &mut Engine) -> bool {

        let image = engine.resource_manager.get_image(self.image_handle_1.unwrap()).unwrap();
        let frame = engine.pixel_buffer.get_frame();
        for y in 0..image.height {
            for x in 0..image.width * 4 {
                let frame_index = x + y * SCREEN_WIDTH * 4;
                let image_index = x + y * image.width * 4;
                frame[frame_index as usize] = image.bytes[image_index as usize];
            }
        }
        true
    }
    fn context(&self) -> &Context {
        &self.ctx
    }
}

fn main() {
    let demo = Demo::new();
    run(demo);
}
