use std::time::Duration;

use winit::event::VirtualKeyCode;

use engine::{
    drawing::blit,
    resource::{Image, ImageResource},
    run,
    types::Vec2,
    Context, Engine, GameState,
};

const SCREEN_WIDTH: u32 = 8;
const SCREEN_HEIGHT: u32 = 8;
const IMAGE_WIDTH: u32 = 4;
const IMAGE_HEIGHT: u32 = 4;

pub struct Demo {
    ctx: Context,
    current_func: TestFn,
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
            current_func: TestFn::Normal,
        }
    }
}

fn get_images() -> (Image, Image) {
    let screen_buf = unsafe {
        [0xFF000000_u32; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize]
            .align_to::<u8>()
            .1
            .to_vec()
    };
    let screen = Image::new(SCREEN_WIDTH, SCREEN_HEIGHT, screen_buf);
    let image_buf = unsafe {
        [0xFFCCBBAA_u32; (IMAGE_WIDTH * IMAGE_HEIGHT) as usize]
            .align_to::<u8>()
            .1
            .to_vec()
    };
    let image = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT, image_buf);
    (screen, image)
}

fn test_blit(x: i32, y: i32) -> Image {
    let (mut screen, image) = get_images();
    let position = Vec2::new(x, y);
    blit(&image, &mut screen, position);
    screen
}

fn test_blit_normal() -> Image {
    test_blit(0, 0)
}
fn test_blit_top_left_offset() -> Image {
    test_blit(-1, -1)
}
fn test_blit_top_right_offset() -> Image {
    test_blit((SCREEN_WIDTH - 1) as i32, -1)
}
fn test_blit_bottom_left_offset() -> Image {
    test_blit(-1, (SCREEN_HEIGHT - 1) as i32)
}
fn test_blit_bottom_right_offset() -> Image {
    test_blit((SCREEN_WIDTH - 1) as i32, (SCREEN_HEIGHT - 1) as i32)
}

#[derive(Copy, Clone, Debug)]
enum TestFn {
    Normal = 0,
    TopLeftOffset = 1,
    TopRightOffset = 2,
    BottomLeftOffset = 3,
    BottomRightOffset = 4,
}

impl From<u8> for TestFn {
    fn from(num: u8) -> Self {
        match num {
            0 => TestFn::Normal,
            1 => TestFn::TopLeftOffset,
            2 => TestFn::TopRightOffset,
            3 => TestFn::BottomLeftOffset,
            4 => TestFn::BottomRightOffset,
            _ => panic!("Invalid number for enum"),
        }
    }
}

impl GameState for Demo {
    fn on_update(&mut self, elapsed_time: Duration, engine: &mut Engine) -> bool {
        engine
            .window
            .set_title(&format!("{}ms", elapsed_time.as_millis()));
        let screen = &mut engine.screen;
        if engine.input.key_released(VirtualKeyCode::M) {
            self.current_func = ((self.current_func as u8 + 1) % 5).into();
        }
        let mut result = match self.current_func {
            TestFn::Normal => test_blit_normal(),
            TestFn::TopLeftOffset => test_blit_top_left_offset(),
            TestFn::TopRightOffset => test_blit_top_right_offset(),
            TestFn::BottomLeftOffset => test_blit_bottom_left_offset(),
            TestFn::BottomRightOffset => test_blit_bottom_right_offset(),
        };
        if engine.input.key_released(VirtualKeyCode::P) {
            println!("{:?}", result.get_buf_mut());
        }
        screen
            .get_buf_u32_mut()
            .copy_from_slice(result.get_buf_u32_mut());
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
