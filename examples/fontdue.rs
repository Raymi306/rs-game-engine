use std::path::Path;
use std::time::Duration;

use fontdue::FontSettings;

use engine::{
    drawing::draw_text,
    resource::FontHandle,
    run,
    types::{Color, Vec2},
    Context, Engine, GameState,
};

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;

pub struct Demo {
    ctx: Context,
    font_handle_1: Option<FontHandle>,
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
            font_handle_1: None,
        }
    }
}

impl GameState for Demo {
    fn on_create(&mut self, engine: &mut Engine) -> bool {
        let settings = FontSettings {
            scale: 10.0,
            ..FontSettings::default()
        };
        self.font_handle_1 = Some(engine.resource_manager.load_font(
            Path::new("resources/fonts/JetbrainsMonoRegular.ttf"),
            settings,
        ));
        true
    }
    fn on_update(&mut self, elapsed_time: Duration, engine: &mut Engine) -> bool {
        engine
            .window
            .set_title(&format!("{}ms", elapsed_time.as_millis()));
        let screen = &mut engine.screen;
        screen.clear(Color::new(0, 0, 0, 255));
        let font = engine
            .resource_manager
            .get_font(self.font_handle_1.unwrap())
            .unwrap();
        let layout = &mut engine.font_helper.default_layout;
        let text = &format!("Render time: {}ms", elapsed_time.as_millis());
        draw_text(
            font,
            layout,
            text,
            40.0,
            Color::new(255, 255, 255, 255),
            screen,
            Vec2 { x: 10, y: 10 },
        );
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
