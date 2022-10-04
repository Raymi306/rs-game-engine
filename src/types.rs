use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for Vec2 {
    type Output = Self;
    fn mul(self, other: i32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl From<Vec2F> for Vec2 {
    fn from(item: Vec2F) -> Self {
        Self {
            x: item.x as i32,
            y: item.y as i32,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2F {
    pub x: f32,
    pub y: f32,
}

impl Add for Vec2F {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2F {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vec2F {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Rect {
    _bottom_left: Vec2,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(p1: Vec2, width: u32, height: u32) -> Self {
        Self {
            _bottom_left: p1,
            width,
            height,
        }
    }
    pub fn offset(&mut self, vector: Vec2) {
        self._bottom_left.x += vector.x;
        self._bottom_left.y += vector.y;
    }
    pub fn bottom_left(&self) -> Vec2 {
        self._bottom_left
    }
    pub fn bottom_right(&self) -> Vec2 {
        Vec2 {
            x: self._bottom_left.x + self.width as i32,
            y: self._bottom_left.y
        }
    }
    pub fn top_left(&self) -> Vec2 {
        Vec2 {
            x: self._bottom_left.x,
            y: self._bottom_left.y - self.height as i32
        }
    }
    pub fn top_right(&self) -> Vec2 {
        Vec2 {
            x: self._bottom_left.x + self.width as i32,
            y: self._bottom_left.y - self.height as i32
        }
    }
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
    pub fn left(&self) -> i32 {
        self._bottom_left.x
    }
    pub fn right(&self) -> i32 {
        self._bottom_left.x + self.width as i32
    }
    pub fn top(&self) -> i32 {
        self._bottom_left.y - self.height as i32
    }
    pub fn bottom(&self) -> i32 {
        self._bottom_left.y
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

impl From<u32> for Color {
    // TODO TEST ME
    fn from(num: u32) -> Self {
        let a = (num & 0x00ff0000) >> 24;
        let b = (num & 0x0000ff00) >> 16;
        let g = (num & 0x000000ff) >> 8;
        let r = num & 0xff000000;
        Color::new(r as u8, g as u8, b as u8, a as u8)
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        ((color.a as u32) << 24) | ((color.b as u32) << 16) | ((color.g as u32) << 8) | color.r as u32
    }
}
