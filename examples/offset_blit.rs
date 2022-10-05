use std::path::Path;
use std::time::Duration;

use winit::event::VirtualKeyCode;

use engine::{
    constants::PIXEL_SIZE,
    drawing::{blit, blit_rect, draw_rectangle_unchecked},
    resource::{ImageHandle, ImageResource},
    run,
    types::{Color, Rect, Vec2},
    Context, Engine, GameState,
};

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;
const PIXELS_WIDTH: u32 = 1024 / 4;
const PIXELS_HEIGHT: u32 = 768 / 4;

#[derive(PartialEq, Debug)]
enum Mode {
    MoveRect,
    MovePosition,
}
pub struct Demo {
    ctx: Context,
    image_handle_1: Option<ImageHandle>,
    rect: Rect,
    position: Vec2,
    mode: Mode,
}

impl Demo {
    pub fn new() -> Self {
        let ctx = Context {
            screen_width: SCREEN_WIDTH,
            screen_height: SCREEN_HEIGHT,
            vsync_enabled: true,
        };
        let sprite_width = 16;
        let sprite_height = 16;
        let rect = Rect::new(
            Vec2 {
                x: sprite_width as i32,
                y: sprite_height as i32,
            },
            sprite_width,
            sprite_height,
        );
        Self {
            ctx,
            image_handle_1: None,
            rect,
            position: Vec2 {
                x: (PIXELS_WIDTH / 2) as i32,
                y: (PIXELS_HEIGHT / 2) as i32,
            },
            mode: Mode::MoveRect,
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
        let buf = screen.get_buf_mut();
        for i in (0..buf.len()).step_by(PIXEL_SIZE as usize) {
            buf[i] = 0;
            buf[i + 1] = 0;
            buf[i + 2] = 0;
            buf[i + 3] = 255;
        }

        let image_1 = engine
            .resource_manager
            .get_image(self.image_handle_1.unwrap())
            .unwrap();
        blit(image_1, screen, Vec2 { x: 0, y: 0 });
        draw_rectangle_unchecked(self.rect, screen, Color::new(255, 255, 0, 255));
        blit_rect(image_1, self.rect, screen, self.position);
        if engine.input.key_released(VirtualKeyCode::M) {
            self.mode = match self.mode {
                Mode::MoveRect => Mode::MovePosition,
                Mode::MovePosition => Mode::MoveRect,
            }
        }
        if engine.input.key_held(VirtualKeyCode::Left) {
            if self.mode == Mode::MoveRect && self.rect.left() - 1 >= 0 {
                self.rect.offset(Vec2 { x: -1, y: 0 });
            } else if self.mode == Mode::MovePosition {
                self.position.x -= 1;
            }
        }
        if engine.input.key_held(VirtualKeyCode::Right) {
            if self.mode == Mode::MoveRect && self.rect.right() + 1 < PIXELS_WIDTH as i32 {
                self.rect.offset(Vec2 { x: 1, y: 0 });
            } else if self.mode == Mode::MovePosition {
                self.position.x += 1;
            }
        }
        if engine.input.key_held(VirtualKeyCode::Up) {
            if self.mode == Mode::MoveRect && self.rect.top() - 1 >= 0 {
                self.rect.offset(Vec2 { x: 0, y: -1 });
            } else if self.mode == Mode::MovePosition {
                self.position.y -= 1;
            }
        }
        if engine.input.key_held(VirtualKeyCode::Down) {
            if self.mode == Mode::MoveRect && self.rect.bottom() + 1 < PIXELS_HEIGHT as i32 {
                self.rect.offset(Vec2 { x: 0, y: 1 });
            } else if self.mode == Mode::MovePosition {
                self.position.y += 1;
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
