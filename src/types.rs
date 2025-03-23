pub use fontdue::FontSettings;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
pub use winit::event::VirtualKeyCode;
pub use winit_input_helper::WinitInputHelper;

macro_rules! impl_common_vec_traits {
    ($name : ident, $type : ident) => {
        impl $name {
            pub fn offset_x(&mut self, val: $type) {
                self.x += val;
            }
            pub fn offset_y(&mut self, val: $type) {
                self.y += val;
            }
            pub fn offset(&mut self, val: $name) {
                *self += val;
            }
        }

        impl Add for $name {
            type Output = Self;
            fn add(self, other: Self) -> Self {
                Self {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        impl AddAssign for $name {
            fn add_assign(&mut self, other: Self) {
                *self = Self {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        impl Sub for $name {
            type Output = Self;
            fn sub(self, other: Self) -> Self {
                Self {
                    x: self.x - other.x,
                    y: self.y - other.y,
                }
            }
        }

        impl SubAssign for $name {
            fn sub_assign(&mut self, other: Self) {
                *self = Self {
                    x: self.x - other.x,
                    y: self.y - other.y,
                }
            }
        }

        impl Mul<$type> for $name {
            type Output = Self;
            fn mul(self, other: $type) -> Self {
                Self {
                    x: self.x * other,
                    y: self.y * other,
                }
            }
        }

        impl MulAssign<$type> for $name {
            fn mul_assign(&mut self, other: $type) {
                *self = Self {
                    x: self.x * other,
                    y: self.y * other,
                }
            }
        }

        impl Div<$type> for $name {
            type Output = Self;
            fn div(self, other: $type) -> Self {
                Self {
                    x: self.x / other,
                    y: self.y / other,
                }
            }
        }

        impl DivAssign<$type> for $name {
            fn div_assign(&mut self, other: $type) {
                *self = Self {
                    x: self.x / other,
                    y: self.y / other,
                }
            }
        }
    };
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl_common_vec_traits!(Vec2, i32);

impl From<Vec2F> for Vec2 {
    fn from(item: Vec2F) -> Self {
        Self {
            x: item.x as i32,
            y: item.y as i32,
        }
    }
}

impl Vec2 {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn magnitude(&self) -> f32 {
        f32::sqrt((self.x * self.x + self.y * self.y) as f32)
    }
    pub fn normalize(&self) -> Vec2F {
        let magnitude = self.magnitude();
        if magnitude > 0.0 {
            Vec2F::from(*self) / magnitude
        } else {
            Vec2F::from(*self)
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec2F {
    pub x: f32,
    pub y: f32,
}

impl_common_vec_traits!(Vec2F, f32);

impl From<Vec2> for Vec2F {
    fn from(item: Vec2) -> Self {
        Self {
            x: item.x as f32,
            y: item.y as f32,
        }
    }
}

impl Vec2F {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn magnitude(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }
    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        if magnitude > 0.0 {
            *self / magnitude
        } else {
            Self { x: 0.0, y: 0.0 }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Rect {
    pub top_left: Vec2,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub const fn new(top_left: Vec2, width: u32, height: u32) -> Self {
        Self {
            top_left,
            width,
            height,
        }
    }
    pub const fn point_intersects(&self, point: Vec2) -> bool {
        point.x >= self.left()
            && point.x <= self.right()
            && point.y >= self.top()
            && point.y <= self.bottom()
    }
    pub const fn intersects(&self, other: &Self) -> bool {
        self.top_left.x < other.top_left.x + other.width as i32
            && self.top_left.x + self.width as i32 > other.top_left.x
            && self.top_left.y < other.top_left.y + other.height as i32
            && self.height as i32 + self.top_left.y > other.top_left.y
    }
    pub fn offset(&mut self, vector: Vec2) {
        self.top_left.x += vector.x;
        self.top_left.y += vector.y;
    }
    pub fn offset_x(&mut self, val: i32) {
        self.top_left.x += val;
    }
    pub fn offset_y(&mut self, val: i32) {
        self.top_left.y += val;
    }
    pub const fn bottom_left(&self) -> Vec2 {
        Vec2 {
            x: self.top_left.x,
            y: self.top_left.y + self.height as i32,
        }
    }
    pub const fn bottom_right(&self) -> Vec2 {
        Vec2 {
            x: self.top_left.x + self.width as i32,
            y: self.top_left.y + self.height as i32,
        }
    }
    pub const fn top_right(&self) -> Vec2 {
        Vec2 {
            x: self.top_left.x + self.width as i32,
            y: self.top_left.y,
        }
    }
    pub const fn area(&self) -> u32 {
        self.width * self.height
    }
    pub const fn left(&self) -> i32 {
        self.top_left.x
    }
    pub const fn right(&self) -> i32 {
        self.top_left.x + self.width as i32
    }
    pub const fn top(&self) -> i32 {
        self.top_left.y
    }
    pub const fn bottom(&self) -> i32 {
        self.top_left.y + self.height as i32
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

/// This is intended to parse RGBA and not necessarily reflect what the format is
impl From<u32> for Color {
    fn from(num: u32) -> Self {
        let r = (num & 0xff000000) >> 24;
        let g = (num & 0x00ff0000) >> 16;
        let b = (num & 0x0000ff00) >> 8;
        let a = num & 0x000000ff;
        Color::new(r as u8, g as u8, b as u8, a as u8)
    }
}

/// This is intended to represent the underlying format
impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        ((color.a as u32) << 24)
            | ((color.b as u32) << 16)
            | ((color.g as u32) << 8)
            | color.r as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_u32() {
        let color_u32: u32 = 0x775533FF;
        let color = Color::from(color_u32);
        assert_eq!(color.r, 0x77, "Red channel incorrect");
        assert_eq!(color.g, 0x55, "Green channel incorrect");
        assert_eq!(color.b, 0x33, "Blue channel incorrect");
        assert_eq!(color.a, 0xFF, "Alpha channel incorrect");
    }

    #[test]
    fn test_u32_from_color() {
        let color = Color::new(0x77, 0x55, 0x33, 0xFF);
        let color_u32 = u32::from(color);
        assert_eq!(color_u32, 0xFF335577);
    }

    #[test]
    fn test_rect_simple() {
        let rect = Rect::new(Vec2::new(0, 0), 10, 10);
        let top_left = rect.top_left;
        let top_right = rect.top_right();
        let bottom_left = rect.bottom_left();
        let bottom_right = rect.bottom_right();
        let area = rect.area();
        let width = rect.width;
        let height = rect.height;
        assert_eq!(top_left, Vec2::new(0, 0));
        assert_eq!(top_right, Vec2::new(10, 0));
        assert_eq!(bottom_left, Vec2::new(0, 10));
        assert_eq!(bottom_right, Vec2::new(10, 10));
        assert_eq!(area, 100);
        assert_eq!(width, 10);
        assert_eq!(height, 10);
    }
}
