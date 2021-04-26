use rulinalg::vector::Vector;

pub mod simple_mat;

pub trait Material {
    fn color(&self, impact: &Vector<f32>) -> Color;
    fn has_secondary_color(&self) -> bool;
}

#[derive(Clone, Copy)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue, alpha: 255 }
    }

    pub fn new_with_alpha(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self { red, green, blue, alpha }
    }

    pub fn new_gray(gray: u8) -> Self {
        Self { red: gray, green: gray, blue: gray, alpha: 255 }
    }

    pub fn new_gray_with_alpha(gray: u8, alpha: u8) -> Self {
        Self { red: gray, green: gray, blue: gray, alpha }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        vec![self.red, self.green, self.blue]
    }
}
