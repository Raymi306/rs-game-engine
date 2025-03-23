use std::time::Duration;

use engine::{
    drawing::{draw_line, draw_rectangle_unchecked, draw_triangle, fill_rectangle_unchecked},
    run,
    types::{Color, Rect, Vec2},
    Context, Engine, GameState,
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
        let screen = &mut engine.screen;
        let r1 = Rect::new(Vec2 { x: 100, y: 100 }, 200, 400);
        let r2 = Rect::new(Vec2 { x: 150, y: 150 }, 300, 100);
        let t1 = (
            Vec2 { x: 200, y: 200 },
            Vec2 { x: 300, y: 300 },
            Vec2 { x: 1000, y: 700 },
        );
        fill_rectangle_unchecked(r1, screen, Color::new(0, 155, 0, 255));
        draw_rectangle_unchecked(r2, screen, Color::new(0, 0, 255, 255));
        draw_triangle(t1.0, t1.1, t1.2, screen, Color::new(255, 0, 0, 255));
        draw_line(
            t1.0,
            Vec2 { x: -1, y: 768 },
            screen,
            Color::new(255, 255, 0, 255),
        );
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
