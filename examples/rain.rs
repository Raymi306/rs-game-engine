use rand::prelude::*;
use std::time::Duration;

use engine::{
    drawing::draw_line,
    run,
    types::{Color, Vec2F as Vec2},
    Context, Engine, GameState,
};

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;
const GRAVITY: f32 = 900.0;

pub struct Demo {
    ctx: Context,
    num_drops: u32,
    rng: ThreadRng,
    raindrops: Vec<(Vec2, Vec2)>,
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
        let num_drops: u32 = 2000;
        let mut rng = thread_rng();
        let mut raindrops = Vec::with_capacity(num_drops as usize);
        for _ in 0..=num_drops {
            let head_x = rng.gen_range(1..SCREEN_WIDTH - 1) as f32;
            let head_y = rng.gen_range(0..SCREEN_HEIGHT - 1) as f32;
            let tail_y = head_y + rng.gen_range(1..100) as f32;
            let droplet = (
                Vec2 {
                    x: head_x,
                    y: head_y,
                },
                Vec2 {
                    x: head_x,
                    y: tail_y,
                },
            );
            raindrops.push(droplet);
        }
        Self {
            ctx,
            num_drops,
            rng,
            raindrops,
        }
    }
}

impl GameState for Demo {
    fn on_update(&mut self, elapsed_time: Duration, engine: &mut Engine) -> bool {
        engine
            .window
            .set_title(&format!("{}ms", elapsed_time.as_millis()));
        let screen = &mut engine.screen;
        screen.clear(Color::new(0, 0, 0, 255));
        for i in 0..700 {
            let droplet = self.raindrops[i];
            draw_line(
                droplet.0.into(),
                droplet.1.into(),
                screen,
                Color::new(100, 100, 150, 255),
            );
        }
        for i in 300..self.num_drops as usize {
            let droplet = self.raindrops[i];
            draw_line(
                droplet.0.into(),
                droplet.1.into(),
                screen,
                Color::new(50, 50, 100, 255),
            );
        }
        for i in 0..700 {
            let droplet = &mut self.raindrops[i];
            let gravity = GRAVITY * elapsed_time.as_secs_f32() * 1.1;
            droplet.0.y += gravity;
            droplet.1.y += gravity;
            if droplet.0.y >= SCREEN_HEIGHT as f32 {
                droplet.0.x = self.rng.gen_range(0..SCREEN_WIDTH) as f32;
                droplet.0.y = -self.rng.gen_range(1..100) as f32;
                droplet.1.x = droplet.0.x;
                droplet.1.y = 0.0;
            }
        }
        for i in 300..self.num_drops as usize {
            let droplet = &mut self.raindrops[i];
            let gravity = GRAVITY * elapsed_time.as_secs_f32();
            droplet.0.y += gravity;
            droplet.1.y += gravity;
            if droplet.0.y >= SCREEN_HEIGHT as f32 {
                droplet.0.x = self.rng.gen_range(0..SCREEN_WIDTH) as f32;
                droplet.0.y = -self.rng.gen_range(1..100) as f32;
                droplet.1.x = droplet.0.x;
                droplet.1.y = 0.0;
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
