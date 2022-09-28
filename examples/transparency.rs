use std::path::Path;
use std::time::Duration;

use engine::{
    Context,
    drawing::{blit, blit_with_alpha},
    Engine,
    GameState,
    resource::{ResourceHandle, ImageResource},
    run,
    types::Vec2,
};

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;
const PIXELS_WIDTH: u32 = 1024/2;
const PIXELS_HEIGHT: u32 = 768/2;

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
        Self { ctx, image_handle_1: None, image_handle_2: None }
    }
}

impl GameState for Demo {
    fn on_create(&mut self, engine: &mut Engine) -> bool {
        engine.resize_buffer(PIXELS_WIDTH, PIXELS_HEIGHT);
        self.image_handle_1 = Some(
            engine
            .resource_manager
            .load_image(Path::new("resources/images/test_pattern_1.bmp"))
        );
        self.image_handle_2 = Some(
            engine
            .resource_manager
            .load_image(Path::new("resources/images/test_logo.png"))
        );
        true
    }
    fn on_update(&mut self, elapsed_time: Duration, engine: &mut Engine) -> bool {
        engine.window.set_title(&format!("{}ms", elapsed_time.as_millis()));
        let image_1 = engine.resource_manager.get_image(self.image_handle_1.unwrap()).unwrap();
        let mut screen = engine.screen();
        {
            for y in (0..PIXELS_HEIGHT).step_by(image_1.height() as usize) {
                for x in (0..PIXELS_WIDTH).step_by(image_1.width() as usize) {
                    blit(image_1, &mut screen, Vec2 { x: x as i32, y: y as i32 });
                }
            }
        }
        let image_2 = engine.resource_manager.get_image(self.image_handle_2.unwrap()).unwrap();
        blit_with_alpha(image_2, &mut screen, Vec2 { x: 0, y: 0 });
        true
    }
    fn context(&self) -> &Context {
        &self.ctx
    }
}

fn main() {
   let game_state = Demo::new();
   run(game_state);
}
