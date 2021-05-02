pub mod simple_mat;
pub mod texture;

use std::ops::{Mul, Add, AddAssign};

pub trait MatProvider {
    fn material(&self, x: f32, y: f32) -> Material;
}

#[derive(Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub const FULL_RED: Color = Color { red: 255, green: 0, blue: 0 };
    pub const FULL_GREEN: Color = Color { red: 0, green: 255, blue: 0};
    pub const FULL_BLUE: Color = Color { red: 0, green: 0, blue: 255};
    pub const RED: Color = Color { red: 170, green: 0, blue: 0};
    pub const GREEN: Color = Color { red: 0, green: 170, blue: 0};
    pub const BLUE: Color = Color { red: 0, green: 0, blue: 170};
    pub const DARK_RED: Color = Color { red: 85, green: 0, blue: 0};
    pub const DARK_GREEN: Color = Color { red: 0, green: 85, blue: 0};
    pub const DARK_BLUE: Color = Color { red: 0, green: 0, blue: 85};

    pub const FULL_WHITE: Color = Color { red: 255, green: 255, blue: 255 };
    pub const WHITE: Color = Color { red: 191, green: 191, blue: 191 };
    pub const GRAY: Color = Color { red: 127, green: 127, blue: 127 };
    pub const BLACK: Color = Color { red: 63, green: 63, blue: 63 };
    pub const FULL_BLACK: Color = Color { red: 0, green: 0, blue: 0 };

    pub const SKY: Color = Color { red: 50, green: 120, blue: 170};

    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn new_gray(gray: u8) -> Self {
        Self { red: gray, green: gray, blue: gray }
    }

    pub fn to_vec(&self) -> [u8; 3] {
        [self.red, self.green, self.blue]
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            red: ((self.red as f32 / 255.0) * (rhs.red as f32 / 255.0) * 255.0).clamp(0.0, 255.0) as u8,
            green: ((self.green as f32 / 255.0) * (rhs.green as f32 / 255.0) * 255.0).clamp(0.0, 255.0) as u8,
            blue: ((self.blue as f32 / 255.0) * (rhs.blue as f32 / 255.0) * 255.0).clamp(0.0, 255.0) as u8,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, coef: f32) -> Self::Output {
        Self {
            red: (self.red as f32 * coef).clamp(0.0, 255.0) as u8,
            green: (self.green as f32 * coef).clamp(0.0, 255.0) as u8,
            blue: (self.blue as f32 * coef).clamp(0.0, 255.0) as u8,
        }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red.saturating_add(rhs.red),
            green: self.green.saturating_add(rhs.green),
            blue: self.blue.saturating_add(rhs.blue),
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.red = self.red.saturating_add(rhs.red);
        self.green = self.green.saturating_add(rhs.green);
        self.blue = self.blue.saturating_add(rhs.blue);
    }
}

#[derive(Clone, Copy)]
pub struct Material {
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: Color,

    pub alpha: u8,
    pub shininess: f32,
}

impl Material {
    pub fn new(ambient: Color, diffuse: Color, specular: Color, alpha: u8, shininess: f32) -> Self {
        Self { ambient, diffuse, specular, alpha, shininess }
    }
}
