pub mod strip_x_mat;
pub mod strip_y_mat;
pub mod simple_mat;
pub mod grid_mat;
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

impl Default for Color {
    fn default() -> Self {
        Color::new_gray(0)
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
    pub reflection: u8,
    pub shininess: f32,
}

impl Material {
    pub fn new(ambient: Color, diffuse: Color, specular: Color, alpha: u8, reflection: u8, shininess: f32) -> Self {
        Self { ambient, diffuse, specular, alpha, reflection, shininess }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: Color::new_gray(63),
            diffuse: Color::new_gray(127),
            specular: Color::new_gray(191),
            alpha: 255,
            reflection: 0,
            shininess: 50.0,
        }
    }
}
