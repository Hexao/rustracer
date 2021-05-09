use crate::material::{MatProvider, Material};

#[derive(Clone, Copy)]
pub struct GridMat {
    materials: [Material; 2],
    rep_x: f32,
    rep_y: f32,
}

impl GridMat {
    pub fn new(mat_1: Material, mat_2: Material, rep_x: usize, rep_y: usize) -> Self {
        assert!(rep_x > 0 && rep_y > 0);
        Self { materials: [mat_1, mat_2], rep_x: rep_x as f32, rep_y: rep_y as f32 }
    }
}

impl MatProvider for GridMat {
    fn material(&self, x: f32, y: f32) -> Material {
        let x = x * self.rep_x % 1.0;
        let y = y * self.rep_y % 1.0;

        if (x <= 0.5) ^ (y <= 0.5) {
            self.materials[0]
        } else {
            self.materials[1]
        }
    }
}
