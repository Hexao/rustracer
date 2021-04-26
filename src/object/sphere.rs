use crate::object::{Movable, Object};
use crate::math::ray::Ray;

use rulinalg::matrix::Matrix;
use rulinalg::vector::Vector;

pub struct Sphere {
    tra: Matrix<f32>,
    inv: Matrix<f32>,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            tra: Matrix::identity(4),
            inv: Matrix::identity(4),
        }
    }
}

impl Movable for Sphere {
    fn tra(&self) -> &Matrix<f32> {
        &self.tra
    }

    fn tra_mut(&mut self) -> &mut Matrix<f32> {
        &mut self.tra
    }

    fn inv(&self) -> &Matrix<f32> {
        &self.inv
    }

    fn inv_mut(&mut self) -> &mut Matrix<f32> {
        &mut self.inv
    }
}

impl Object for Sphere {
    fn intersect(&self, ray: Ray, impact: &mut Vector<f32>) -> bool {
        let ray = self.global_to_local_ray(ray);
        let origin = ray.origin();
        let vector = ray.vector();

        let a = vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2];
        let b = 2.0 * (vector[0] * origin[0] + vector[1] * origin[1] + vector[2] * origin[2]);
        let c = origin[0] * origin[0] + origin[1] * origin[1] + origin[2] * origin[2] - 1.0;
        let d = b * b - 4.0 * a * c;

        if d >= 0. {
            let d_sqrt = d.sqrt();
            let x1 = (-b - d_sqrt) / (2.0 * a);
            let x2 = (-b + d_sqrt) / (2.0 * a);

            if x1 < 0. && x2 < 0. {
                return false;
            } else if x1 < x2 && x1 >= 0. {
                *impact = origin + vector * x1;
            } else {
                *impact = origin + vector * x2;
            }
        }

        *impact = self.local_to_global_point(impact.clone());

        d >= 0.
    }
}
