use crate::material::{MatProvider, Material};

#[derive(Clone, Copy)]
pub struct StripYMat {
    materials: [Material; 2],
    rep: f32,
}

impl StripYMat {
    pub fn new(mat_1: Material, mat_2: Material, rep: usize) -> Self {
        assert!(rep > 0);
        Self { materials: [mat_1, mat_2], rep: rep as f32 }
    }
}

impl MatProvider for StripYMat {
    fn material(&self, _x: f32, y: f32) -> Material {
        let y = y * self.rep % 1.0;

        if y <= 0.5 {
            self.materials[0]
        } else {
            self.materials[1]
        }
    }
}
