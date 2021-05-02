use crate::material::{MatProvider, Material};

#[derive(Clone, Copy)]
pub struct SimpleMat {
    material: Material
}

impl SimpleMat {
    pub fn new(material: Material) -> Self {
        Self { material }
    }
}

impl MatProvider for SimpleMat {
    fn material(&self, _x: f32, _y: f32) -> Material {
        self.material
    }
}
