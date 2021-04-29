use rulinalg::norm::Euclidean;
use rulinalg::vector::Vector;

#[derive(Clone)]
pub struct Ray {
    origin: Vector<f32>,
    vector: Vector<f32>,
}

impl Ray {
    pub fn new(ox: f32, oy: f32, oz: f32, vx: f32, vy: f32, vz: f32) -> Self {
        Ray {
            origin: Vector::new(vec![ox, oy, oz, 1.0]),
            vector: Vector::new(vec![vx, vy, vz, 0.0]),
        }
    }

    pub fn origin(&self) -> &Vector<f32> {
        &self.origin
    }

    pub fn vector(&self) -> &Vector<f32> {
        &self.vector
    }

    pub fn normalized(self) -> Self {
        let ray_norm = self.vector.norm(Euclidean);

        Ray {
            origin: self.origin,
            vector: self.vector / ray_norm,
        }
    }

    pub fn consume(self) -> (Vector<f32>, Vector<f32>) {
        (self.origin, self.vector)
    }
}
