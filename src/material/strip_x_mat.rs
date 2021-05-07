use crate::material::{MatProvider, Material};

#[derive(Clone, Copy)]
pub struct StripXMat {
    materials: [Material; 2],
    rep: f32,
}

impl StripXMat {
    pub fn new(mat_1: Material, mat_2: Material, rep: usize) -> Self {
        assert!(rep > 0);
        Self { materials: [mat_1, mat_2], rep: rep as f32 }
    }
}

impl MatProvider for StripXMat {
    fn material(&self, x: f32, _y: f32) -> Material {
        let x = x * self.rep % 1.0;

        if x <= 0.5 {
            self.materials[0]
        } else {
            self.materials[1]
        }
    }
}
