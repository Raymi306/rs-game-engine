use std::path::Path;
use std::time::Duration;

use engine::{
    drawing::blit,
    resource::{ImageHandle, ImageResource},
    run,
    types::Vec2,
    Context, Engine, GameState,
};

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;
const PIXELS_WIDTH: u32 = 1024 / 2;
const PIXELS_HEIGHT: u32 = 768 / 2;

pub struct Demo {
    ctx: Context,
    image_handle_1: Option<ImageHandle>,
}

impl Default for Demo {
    fn default() -> Self {
        Self::new()
    }
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
        }
    }
}

impl GameState for Demo {
    fn on_create(&mut self, engine: &mut Engine) -> bool {
        engine.resize_buffer(PIXELS_WIDTH, PIXELS_HEIGHT);
        self.image_handle_1 = Some(
            engine
                .resource_manager
                .load_image(Path::new("resources/images/test_pattern_1.bmp")),
        );
        true
    }
    fn on_update(&mut self, elapsed_time: Duration, engine: &mut Engine) -> bool {
        engine
            .window
            .set_title(&format!("{}ms", elapsed_time.as_millis()));
        let screen = &mut engine.screen;
        let image_1 = engine
            .resource_manager
            .get_image(self.image_handle_1.unwrap())
            .unwrap();
        for y in (0..PIXELS_HEIGHT).step_by(image_1.height() as usize) {
            for x in (0..PIXELS_WIDTH).step_by(image_1.width() as usize) {
                blit(
                    image_1,
                    screen,
                    Vec2 {
                        x: x as i32,
                        y: y as i32,
                    },
                );
            }
        }
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
