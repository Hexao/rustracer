use std::ops::Mul;
use rulinalg::matrix::Matrix;
use rulinalg::{vector::Vector, vector};

#[derive(Clone)]
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

    pub fn origin(&self) -> Vector<f32> {
        self.origin.clone()
    }

    pub fn ray(&self) -> Vector<f32> {
        self.ray.clone()
    }

    pub fn normalized(&self) -> Self {
        let ray_norm = self.ray.norm(rulinalg::norm::Euclidean);

        Ray {
            origin: self.origin.clone(),
            ray: self.ray.clone() / ray_norm,
        }
    }
}

impl Mul<Ray> for &Matrix<f32> {
    type Output = Ray;

    fn mul(self, rhs: Ray) -> Self::Output {
        let Ray {mut origin, mut ray} = rhs;

        origin = vector![
            origin[0],
            origin[1],
            origin[2],
            1.0
        ];
        ray = vector![
            ray[0],
            ray[1],
            ray[2],
            0.0
        ];

        origin = self * origin;
        ray = self * ray;

        Ray::new(
            origin[0] / origin[3], origin[1] / origin[3], origin[2] / origin[3],
            ray[0], ray[1], ray[2]
        )
    }
}
