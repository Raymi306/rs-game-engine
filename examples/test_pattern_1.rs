use std::time::Duration;

use engine::{
    Context,
    Engine,
    GameState,
    resource::ImageResource,
    run,
};

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;

pub struct Demo {
    ctx: Context,
}

impl Demo {
    pub fn new() -> Self {
        let ctx = Context {
            screen_width: SCREEN_WIDTH,
            screen_height: SCREEN_HEIGHT,
            vsync_enabled: false,
        };
        Self { ctx }
    }
}

impl GameState for Demo {
    fn on_update(&mut self, elapsed_time: Duration, engine: &mut Engine) -> bool {
        engine
            .window
            .set_title(&format!("{}ms", elapsed_time.as_millis()));
        let buf = engine.screen.get_buf_mut();
        for (i, pixel) in buf.chunks_exact_mut(4).enumerate() {
            let rgba: [u8; 4] = [
                (i % 255).try_into().unwrap(),
                (i % 255).try_into().unwrap(),
                (i % 255).try_into().unwrap(),
                0xff,
            ];
            pixel.copy_from_slice(&rgba);
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
