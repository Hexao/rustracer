use rulinalg::vector::Vector;

use crate::material::{Material, Color};

#[derive(Clone, Copy)]
pub struct SimpleMat {
    color: Color
}

impl SimpleMat {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Material for SimpleMat {
    fn color(&self, _impact: &Vector<f32>) -> Color {
        self.color
    }

    fn has_secondary_color(&self) -> bool {
        false
    }
}
