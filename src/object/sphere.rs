use crate::object::{Movable, Object};
use crate::math::ray::Ray;

use rulinalg::matrix::Matrix;
use rulinalg::vector;

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
    fn global_to_local(&self, ray: Ray) -> Ray {
        &self.inv * ray
    }

    fn local_to_global(&self, ray: Ray) -> Ray {
        &self.tra * ray
    }

    fn transform(&mut self, transform: Matrix<f32>) {
        self.tra = transform * &self.tra;
        self.inv = self.tra.clone().inverse().unwrap();
    }
}

impl Object for Sphere {
    fn intersect(&self, ray: Ray, _impact: &mut vector::Vector<f32>) -> bool {
        let ray = self.global_to_local(ray);
        let o = ray.origin();
        let r = ray.ray();

        let a = r[0] * r[0] + r[1] * r[1] + r[2] * r[2];
        let b = 2.0 * (r[0] * o[0] + r[1] * o[1] + r[2] * o[2]);
        let c = o[0] * o[0] + o[1] * o[1] + o[2] * o[2] - 1.0;
        let d = b * b - 4.0 * a * c;

        d >= 0.
    }
}
