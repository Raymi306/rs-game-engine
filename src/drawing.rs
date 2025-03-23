use std::cmp;

use fontdue::layout::{Layout, LayoutSettings, TextStyle};
use fontdue::Font;

use crate::constants::PIXEL_SIZE;
use crate::resource::{Image, ImageResource};
use crate::types::{Color, Rect, Vec2};

pub fn blit(src: &impl ImageResource, dst: &mut impl ImageResource, position: Vec2) {
    // this function taken in part from blit crate
    let src_width = src.width();
    let src_height = src.height();
    let dst_width = dst.width();

    let src_buf = src.get_buf_u32();
    let dst_buf = dst.get_buf_u32_mut();

    let dst_size = (
        dst_width as i32,
        (dst_buf.len() / dst_width as usize) as i32,
    );

    let min_x = cmp::max(-position.x, 0);
    let min_y = cmp::max(-position.y, 0);
    let max_x = cmp::min(dst_size.0 - position.x, src_width as i32);
    let max_y = cmp::min(dst_size.1 - position.y, src_height as i32);

    for y in min_y..max_y {
        for x in min_x..max_x {
            let index = // TODO rethink these casts?
                ((x + position.x) + ((y + position.y) * dst_width as i32)) as usize;
            let src_index = (x + y * src_width as i32) as usize;
            dst_buf[index] = src_buf[src_index];
        }
    }
}

pub fn blit_with_alpha(src: &impl ImageResource, dst: &mut impl ImageResource, position: Vec2) {
    // how to blend with alpha https://stackoverflow.com/a/64655571/9057528
    let src_width = src.width();
    let src_height = src.height();
    let dst_width = dst.width();

    let src_buf = src.get_buf_u32();
    let dst_buf = dst.get_buf_u32_mut();

    let dst_size = (
        dst_width as i32,
        (dst_buf.len() / dst_width as usize) as i32,
    );

    let min_x = cmp::max(-position.x, 0);
    let min_y = cmp::max(-position.y, 0);
    let max_x = cmp::min(dst_size.0 - position.x, src_width as i32);
    let max_y = cmp::min(dst_size.1 - position.y, src_height as i32);

    for y in min_y..max_y {
        for x in min_x..max_x {
            let dst_index = (x + position.x + ((y + position.y) * dst_width as i32)) as usize;
            let src_index = (x + y * src_width as i32) as usize;
            let out = blend_alpha(src_buf[src_index], dst_buf[dst_index]);
            dst_buf[dst_index] = out;
        }
    }
}

pub fn blit_rect(
    src: &impl ImageResource,
    src_rect: Rect,
    dst: &mut impl ImageResource,
    position: Vec2,
) {
    // stolen shamelessly from OneLoneCoder's PixelGameEngine with bounds checking that ended up
    // looking like blit crate's
    let src_width = src.width() as i32;
    let src_height = src.height() as i32;
    let dst_width = dst.width() as i32;
    let dst_height = dst.height() as i32;
    let min_x = cmp::max(-position.x, 0);
    let min_y = cmp::max(-position.y, 0);
    let max_x = cmp::min(dst_width - position.x, src_rect.width as i32);
    let max_y = cmp::min(dst_height - position.y, src_rect.height as i32);
    if src_rect.right() > src_width || src_rect.bottom() > src_height {
        return;
    }
    let src_buf = src.get_buf_u32();
    let dst_buf = dst.get_buf_u32_mut();

    for y in min_y..max_y {
        for x in min_x..max_x {
            let dst_index = (position.x + x + (y + position.y) * dst_width) as usize;
            let src_index =
                (x + src_rect.top_left.x + (y + src_rect.top_left.y) * src_width) as usize;
            dst_buf[dst_index] = src_buf[src_index];
        }
    }
}

pub fn blit_rect_with_alpha(
    src: &impl ImageResource,
    src_rect: Rect,
    dst: &mut impl ImageResource,
    position: Vec2,
) {
    // stolen shamelessly from OneLoneCoder's PixelGameEngine with bounds checking that ended up
    // looking like blit crate's
    let src_width = src.width() as i32;
    let src_height = src.height() as i32;
    let dst_width = dst.width() as i32;
    let dst_height = dst.height() as i32;
    let min_x = cmp::max(-position.x, 0);
    let min_y = cmp::max(-position.y, 0);
    let max_x = cmp::min(dst_width - position.x, src_rect.width as i32);
    let max_y = cmp::min(dst_height - position.y, src_rect.height as i32);
    if src_rect.right() > src_width || src_rect.bottom() > src_height {
        return;
    }
    let src_buf = src.get_buf_u32();
    let dst_buf = dst.get_buf_u32_mut();

    for y in min_y..max_y {
        for x in min_x..max_x {
            let dst_index = (position.x + x + (y + position.y) * dst_width) as usize;
            let src_index =
                (x + src_rect.top_left.x + (y + src_rect.top_left.y) * src_width) as usize;
            let out = blend_alpha(src_buf[src_index], dst_buf[dst_index]);
            dst_buf[dst_index] = out;
        }
    }
}

#[inline]
fn plot_unchecked(x: u32, y: u32, dst: &mut impl ImageResource, color: Color) {
    let dst_width = dst.width();
    let index = ((x + y * dst_width) * PIXEL_SIZE) as usize;
    let dst_buf = dst.get_buf_mut();
    dst_buf[index] = color.r;
    dst_buf[index + 1] = color.g;
    dst_buf[index + 2] = color.b;
    dst_buf[index + 3] = color.a;
}

#[inline]
fn plot(x: i32, y: i32, dst: &mut impl ImageResource, color: Color) {
    let dst_width = dst.width();
    let dst_height = dst.height();
    if x < 0 || y < 0 || x >= dst_width as i32 || y >= dst_height as i32 {
        return;
    }
    let index = ((x + y * dst_width as i32) * PIXEL_SIZE as i32) as usize;
    let dst_buf = dst.get_buf_mut();
    dst_buf[index] = color.r;
    dst_buf[index + 1] = color.g;
    dst_buf[index + 2] = color.b;
    dst_buf[index + 3] = color.a;
}

pub fn draw_line(start: Vec2, end: Vec2, dst: &mut impl ImageResource, color: Color) {
    // Bresenham's algorithm shamelessly stolen from wikipedia's pseudocode
    let distance_x = (end.x - start.x).abs();
    let slope_x;
    if start.x < end.x {
        slope_x = 1;
    } else {
        slope_x = -1;
    }
    let distance_y = -(end.y - start.y).abs();
    let slope_y;
    if start.y < end.y {
        slope_y = 1;
    } else {
        slope_y = -1;
    }
    let mut error = distance_x + distance_y;
    let mut cur_x = start.x;
    let mut cur_y = start.y;
    loop {
        plot(cur_x, cur_y, dst, color);
        if cur_x == end.x && cur_y == end.y {
            break;
        }
        let error_2 = 2 * error;
        if error_2 >= distance_y {
            if cur_x == end.x {
                break;
            }
            error += distance_y;
            cur_x += slope_x;
        }
        if error_2 <= distance_x {
            if cur_y == end.y {
                break;
            }
            error += distance_x;
            cur_y += slope_y;
        }
    }
}

pub fn draw_triangle(p1: Vec2, p2: Vec2, p3: Vec2, dst: &mut impl ImageResource, color: Color) {
    draw_line(p1, p2, dst, color);
    draw_line(p2, p3, dst, color);
    draw_line(p3, p1, dst, color);
}

pub fn draw_vertical_unchecked(p1: Vec2, length: u32, dst: &mut impl ImageResource, color: Color) {
    // TODO rethink casts
    for y in p1.y..length as i32 + p1.y {
        plot_unchecked(p1.x as u32, y as u32, dst, color);
    }
}

pub fn draw_horizontal_unchecked(
    p1: Vec2,
    length: u32,
    dst: &mut impl ImageResource,
    color: Color,
) {
    // TODO rethink casts
    for x in p1.x..(length as i32 + p1.x) {
        plot_unchecked(x as u32, p1.y as u32, dst, color);
    }
}

pub fn draw_vertical(p1: Vec2, length: u32, dst: &mut impl ImageResource, color: Color) {
    // TODO rethink casts
    for y in p1.y..length as i32 + p1.y {
        plot(p1.x, y, dst, color);
    }
}

pub fn draw_horizontal(p1: Vec2, length: u32, dst: &mut impl ImageResource, color: Color) {
    // TODO rethink casts
    for x in p1.x..(length as i32 + p1.x) {
        plot(x, p1.y, dst, color);
    }
}

pub fn draw_rectangle_unchecked(rect: Rect, dst: &mut impl ImageResource, color: Color) {
    let height = rect.height;
    let width = rect.width;
    draw_vertical_unchecked(rect.top_left, height, dst, color);
    draw_horizontal_unchecked(rect.bottom_left(), width, dst, color);
    draw_vertical_unchecked(
        rect.top_right(),
        height + 1, // Why + 1??
        dst,
        color,
    );
    draw_horizontal_unchecked(rect.top_left, width, dst, color);
}

pub fn draw_rectangle(rect: Rect, dst: &mut impl ImageResource, color: Color) {
    let height = rect.height;
    let width = rect.width;
    draw_vertical(rect.top_left, height, dst, color);
    draw_horizontal(rect.bottom_left(), width, dst, color);
    draw_vertical(
        rect.top_right(),
        height + 1, // Why + 1??
        dst,
        color,
    );
    draw_horizontal(rect.top_left, width, dst, color);
}

pub fn fill_rectangle_unchecked(rect: Rect, dst: &mut impl ImageResource, color: Color) {
    for y in rect.top()..rect.height as i32 {
        draw_horizontal_unchecked(Vec2::new(rect.left(), y), rect.width, dst, color);
    }
}

pub fn fill_rectangle(rect: Rect, dst: &mut impl ImageResource, color: Color) {
    for y in rect.top()..rect.height as i32 {
        draw_horizontal(Vec2::new(rect.left(), y), rect.width, dst, color);
    }
}

pub fn draw_text(
    font: &Font,
    layout: &mut Layout,
    text: &str,
    size: f32,
    color: Color,
    screen: &mut impl ImageResource,
    offset: Vec2,
) {
    // Note that the alpha channel in color is currently ignored
    layout.reset(&LayoutSettings {
        ..LayoutSettings::default()
    });
    layout.append(&[font], &TextStyle::new(text, size, 0));
    for glyph in layout.glyphs() {
        let (metrics, coverage) = font.rasterize(glyph.parent, size);
        let glyph_image_buf_32 = coverage
            .iter()
            .map(|mask| mask_to_u32(color.r, color.g, color.b, *mask))
            .collect::<Vec<u32>>();
        let glyph_image_buf = unsafe { glyph_image_buf_32.align_to::<u8>().1.to_vec() };
        let glyph_image = Image::new(metrics.width as u32, metrics.height as u32, glyph_image_buf);
        blit_with_alpha(
            &glyph_image,
            screen,
            Vec2 {
                x: glyph.x as i32 + offset.x,
                y: glyph.y as i32 + offset.y,
            },
        );
    }
}

pub fn draw_text_to_image(
    font: &Font,
    layout: &mut Layout,
    text: &str,
    size: f32,
    color: Color,
) -> Image {
    // Note that the alpha channel in color is currently ignored
    layout.reset(&LayoutSettings {
        ..LayoutSettings::default()
    });
    layout.append(&[font], &TextStyle::new(text, size, 0));
    let glyphs = layout.glyphs();
    let last_glyph = glyphs.last().unwrap();
    let width = last_glyph.x as usize + last_glyph.width;
    let line = layout.lines().unwrap()[0];
    let height = (line.max_ascent - line.min_descent) as usize;
    let mut result_image = Image::new(
        width as u32,
        height as u32,
        vec![0; width * height * PIXEL_SIZE as usize],
    );
    for glyph in glyphs {
        let (metrics, coverage) = font.rasterize(glyph.parent, size);
        let glyph_image_buf_32 = coverage
            .iter()
            .map(|mask| mask_to_u32(color.r, color.g, color.b, *mask))
            .collect::<Vec<u32>>();
        let glyph_image_buf = unsafe { glyph_image_buf_32.align_to::<u8>().1.to_vec() };
        let glyph_image = Image::new(metrics.width as u32, metrics.height as u32, glyph_image_buf);
        blit_with_alpha(
            &glyph_image,
            &mut result_image,
            Vec2 {
                x: glyph.x as i32,
                y: glyph.y as i32,
            },
        );
    }
    result_image
}

#[inline]
fn blend_alpha(src: u32, dst: u32) -> u32 {
    let src_r = src & 0xFF;
    let src_g = (src & 0xFF00) >> 8;
    let src_b = (src & 0xFF0000) >> 16;
    let src_a = (src & 0xFF000000) >> 24;

    let dst_r = dst & 0xFF;
    let dst_g = (dst & 0xFF00) >> 8;
    let dst_b = (dst & 0xFF0000) >> 16;
    let dst_a = (dst & 0xFF000000) >> 24;

    let r_out = (src_r * src_a / 255) + (dst_r * dst_a * (255 - src_a) / (255 * 255));
    let g_out = (src_g * src_a / 255) + (dst_g * dst_a * (255 - src_a) / (255 * 255));
    let b_out = (src_b * src_a / 255) + (dst_b * dst_a * (255 - src_a) / (255 * 255));
    let a_out = src_a + (dst_a * (255 - src_a) / 255);

    r_out | (g_out << 8) | (b_out << 16) | (a_out << 24)
}

#[inline]
pub fn mask_to_u32(r: u8, g: u8, b: u8, mask: u8) -> u32 {
    ((mask as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | b as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const SCREEN_WIDTH: u32 = 8;
    const SCREEN_HEIGHT: u32 = 8;
    const IMAGE_WIDTH: u32 = 4;
    const IMAGE_HEIGHT: u32 = 4;

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

    #[test]
    fn test_blit_normal() {
        let result = test_blit(0, 0);
        let expected = [
            170, 187, 204, 255, 170, 187, 204, 255, 170, 187, 204, 255, 170, 187, 204, 255, 0, 0,
            0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 170, 187, 204, 255, 170, 187, 204,
            255, 170, 187, 204, 255, 170, 187, 204, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255,
            0, 0, 0, 255, 170, 187, 204, 255, 170, 187, 204, 255, 170, 187, 204, 255, 170, 187,
            204, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 170, 187, 204, 255,
            170, 187, 204, 255, 170, 187, 204, 255, 170, 187, 204, 255, 0, 0, 0, 255, 0, 0, 0, 255,
            0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0,
            0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0,
            0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255,
        ] as [u8; 256];
        assert_eq!(*result.get_buf(), expected);
    }
    #[test]
    fn test_blit_top_left_offset() {
        let result = test_blit(-1, -1);
        let expected = [
            170, 187, 204, 255, 170, 187, 204, 255, 170, 187, 204, 255, 0, 0, 0, 255, 0, 0, 0, 255,
            0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 170, 187, 204, 255, 170, 187, 204, 255, 170,
            187, 204, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255,
            170, 187, 204, 255, 170, 187, 204, 255, 170, 187, 204, 255, 0, 0, 0, 255, 0, 0, 0, 255,
            0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0,
            0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0,
            0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255,
        ] as [u8; 256];
        assert_eq!(*result.get_buf(), expected);
    }
    #[test]
    fn test_blit_top_right_offset() {
        let result = test_blit((SCREEN_WIDTH - 1) as i32, -1);
        let expected = [
            0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0,
            0, 0, 255, 170, 187, 204, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255,
            0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 170, 187, 204, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 170, 187,
            204, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0,
            0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255,
        ] as [u8; 256];
        assert_eq!(*result.get_buf(), expected);
    }
    #[test]
    fn test_blit_bottom_left_offset() {
        let result = test_blit(-1, (SCREEN_HEIGHT - 1) as i32);
        let expected = [
            0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0,
            0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0,
            0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 170, 187, 204, 255, 170, 187, 204, 255, 170, 187, 204, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255,
        ] as [u8; 256];
        assert_eq!(*result.get_buf(), expected);
    }
    #[test]
    fn test_blit_bottom_right_offset() {
        let result = test_blit((SCREEN_WIDTH - 1) as i32, (SCREEN_HEIGHT - 1) as i32);
        let expected = [
            0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0,
            0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0,
            0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 170, 187, 204, 255,
        ] as [u8; 256];
        assert_eq!(*result.get_buf(), expected);
    }

    //TODO
    /*
    fn test_blit_rect(rect: Rect, x: i32, y: i32) -> Image {
        let (mut screen, image) = get_images();
        let position = Vec2::new(x, y);
        blit_rect(&image, rect, &mut screen, position);
        screen
    }
    */
}
