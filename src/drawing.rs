use std::cmp;

use crate::resource::ImageResource;
use crate::constants::PIXEL_SIZE;
use crate::types::{Vec2, Color};

pub fn blit(src: &impl ImageResource, dst: &mut impl ImageResource, position: Vec2) {
    // this function taken in part from blit crate
    let src_width = src.width();
    let src_height = src.height();
    let dst_width = dst.width();

    let src_buf = src.get_buf();
    let dst_buf = dst.get_buf_mut();

    let dst_size = (dst_width as i32, (dst_buf.len() as usize / PIXEL_SIZE as usize / dst_width as usize) as i32);

    let min_x = cmp::max(-position.x * PIXEL_SIZE as i32, 0);
    let min_y = cmp::max(-position.y as i32, 0);
    let max_x = cmp::min(dst_size.0 - position.x, src_width as i32);
    let max_y = cmp::min(dst_size.1 - position.y, src_height as i32);

    for y in min_y..max_y {
        for x in (min_x..max_x * PIXEL_SIZE as i32).step_by(PIXEL_SIZE as usize) {
            let index = // TODO rethink these casts?
                ((x as i32 + (position.x * PIXEL_SIZE as i32))
                + ((y as i32 + position.y) * dst_width as i32)
                * PIXEL_SIZE as i32) as usize;
            let src_index = (x + y * (src_width * PIXEL_SIZE) as i32) as usize;
            dst_buf[index]     = src_buf[src_index];
            dst_buf[index + 1] = src_buf[src_index + 1];
            dst_buf[index + 2] = src_buf[src_index + 2];
            dst_buf[index + 3] = src_buf[src_index + 3];
        }
    }
}

pub fn blit_with_alpha(src: &impl ImageResource, dst: &mut impl ImageResource, position: Vec2) {
    // how to blend with alpha https://stackoverflow.com/a/64655571/9057528
    let src_width = src.width();
    let src_height = src.height();
    let dst_width = dst.width();

    let src_buf = src.get_buf();
    let dst_buf = dst.get_buf_mut();

    let dst_size = (dst_width as i32, (dst_buf.len() as usize / PIXEL_SIZE as usize / dst_width as usize) as i32);

    let min_x = cmp::max(-position.x * PIXEL_SIZE as i32, 0);
    let min_y = cmp::max(-position.y as i32, 0);
    let max_x = cmp::min(dst_size.0 - position.x, src_width as i32);
    let max_y = cmp::min(dst_size.1 - position.y, src_height as i32);

    for y in min_y..max_y {
        for x in (min_x..max_x * PIXEL_SIZE as i32).step_by(PIXEL_SIZE as usize) {
            let index = // TODO rethink these casts?
                ((x as i32 + (position.x * PIXEL_SIZE as i32))
                + ((y as i32 + position.y) * dst_width as i32)
                * PIXEL_SIZE as i32) as usize;
            let src_index = (x + y * (src_width * PIXEL_SIZE) as i32) as usize;

            let src_r = src_buf[src_index] as u32;
            let src_g = src_buf[src_index + 1] as u32;
            let src_b = src_buf[src_index + 2] as u32;
            let src_a = src_buf[src_index + 3] as u32;

            let dst_r = dst_buf[index] as u32;
            let dst_g = dst_buf[index + 1] as u32;
            let dst_b = dst_buf[index + 2] as u32;
            let dst_a = dst_buf[index + 3] as u32;

            /* divide by zero
            let r_out = (src_r * src_a + dst_r * dst_a * (255 - src_a) / 255) / a_out;
            let g_out = (src_g * src_a + dst_g * dst_a * (255 - src_a) / 255) / a_out;
            let b_out = (src_b * src_a + dst_b * dst_a * (255 - src_a) / 255) / a_out;
            let a_out = src_a + (dst_a * (255 - src_a));
            */
            let r_out = (src_r * src_a / 255) + (dst_r * dst_a * (255 - src_a) / (255 * 255));
            let g_out = (src_g * src_a / 255) + (dst_g * dst_a * (255 - src_a) / (255 * 255));
            let b_out = (src_b * src_a / 255) + (dst_b * dst_a * (255 - src_a) / (255 * 255));
            let a_out = src_a + (dst_a * (255 - src_a) / 255);

            dst_buf[index]     = r_out as u8;
            dst_buf[index + 1] = g_out as u8;
            dst_buf[index + 2] = b_out as u8;
            dst_buf[index + 3] = a_out as u8;
        }
    }
}

// TODO how to blit with a source rect, so we can use spritemaps?

#[inline]
fn plot_unchecked(x: u32, y: u32, dst: &mut impl ImageResource, color: Color) {
    let dst_width = dst.width();
    let index = ((x + y * dst_width) * PIXEL_SIZE) as usize;
    let dst_buf = dst.get_buf_mut();
    dst_buf[index]     = color.r;
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
    dst_buf[index]     = color.r;
    dst_buf[index + 1] = color.g;
    dst_buf[index + 2] = color.b;
    dst_buf[index + 3] = color.a;
}

/*
pub fn draw_line(start: Vec2, end: Vec2, dst: impl ImageResource, color: Color) {
    // bresenham's algorithm shamelessly stolen from wikipedia's pseudocode
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
            error = error + distance_y;
            cur_x = cur_x + slope_x;
        }
        if error_2 <= distance_x {
            if cur_y == end.y {
                break;
            }
            error = error + distance_x;
            cur_y = cur_y + slope_y;
        }
    }
}

pub fn draw_triangle(p1: Vec2, p2: Vec2, p3: Vec2, dst: impl ImageResource, color: Color) {
    draw_line(p1, p2, dst, color);
    draw_line(p2, p3, dst, color);
    draw_line(p3, p1, dst, color);
}

pub fn draw_vertical_unchecked(p1: Vec2, length: u32, dst: impl ImageResource, color: Color) {
    let dst_width = dst.width();
    for y in p1.y..length as i32 + p1.y {
        dst.buf[((p1.x + dst_width as i32 * y) * PIXEL_SIZE as i32)     as usize] = color.r;
        dst.buf[((p1.x + dst_width as i32 * y) * PIXEL_SIZE as i32 + 1) as usize] = color.g;
        dst.buf[((p1.x + dst_width as i32 * y) * PIXEL_SIZE as i32 + 2) as usize] = color.b;
        dst.buf[((p1.x + dst_width as i32 * y) * PIXEL_SIZE as i32 + 3) as usize] = color.a;
    }
}

pub fn draw_horizontal_unchecked(p1: Vec2, length: u32, dst: impl ImageResource, color: Color) {
    let dst_width = dst.width();
    for x in (p1.x * PIXEL_SIZE as i32..(length as i32 + p1.x) * PIXEL_SIZE as i32).step_by(PIXEL_SIZE as usize) {
        dst.buf[(x +     (p1.y * dst_width as i32 * PIXEL_SIZE as i32)) as usize] = color.r;
        dst.buf[(x + 1 + (p1.y * dst_width as i32 * PIXEL_SIZE as i32)) as usize] = color.g;
        dst.buf[(x + 2 + (p1.y * dst_width as i32 * PIXEL_SIZE as i32)) as usize] = color.b;
        dst.buf[(x + 3 + (p1.y * dst_width as i32 * PIXEL_SIZE as i32)) as usize] = color.a;
    }
}

pub fn draw_rectangle_unchecked(bottom_left: Vec2, top_right: Vec2, dst: impl ImageResource, color: Color) {
    let height = (top_right.y - bottom_left.y) as u32;
    let width = (top_right.x - bottom_left.x) as u32;
    draw_vertical_unchecked(bottom_left, height, dst, color);
    draw_horizontal_unchecked(bottom_left, width, dst, color);
    draw_vertical_unchecked(Vec2 { x: top_right.x, y: top_right.y - height as i32 }, height, dst, color);
    draw_horizontal_unchecked(Vec2 { x: bottom_left.x, y: top_right.y }, width, dst, color);
}
*/
