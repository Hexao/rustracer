use rulinalg::vector::Vector;

pub struct Ray {
    origin: Vector<f32>,
    ray: Vector<f32>,
}

impl Ray {
    pub fn new(ox: f32, oy: f32, oz: f32, rx: f32, ry: f32, rz: f32) -> Self {
        Ray {
            origin: Vector::new(vec![ox, oy, oz]),
            ray: Vector::new(vec![rx, ry, rz]),
        }
    }

    pub fn normalized(&self) -> Self {
        let ray_norm = self.ray.norm(rulinalg::norm::Euclidean);

        Ray {
            origin: self.origin.clone(),
            ray: self.ray.clone() / ray_norm,
        }
    }
}
