use crate::object::Object;
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

impl Object for Sphere {
    fn global_to_local(&self, ray: Ray) -> Ray {
        let mut o = ray.origin();
        let mut r = ray.ray();

        o = vector![o[0], o[1], o[2], 1.0];
        o = &self.inv * o;

        r = vector![r[0], r[1], r[2], 0.0];
        r = &self.inv * r;

        Ray::new(o[0] / o[3], o[1] / o[3], o[2] / o[3], r[0], r[1], r[2])
    }

    fn local_to_global(&self, ray: Ray) -> Ray {
        ray
    }

    fn transform(&mut self, transform: Matrix<f32>) {
        self.tra = transform * &self.tra;
        self.inv = self.tra.clone().inverse().unwrap();
    }

    fn intersect(&self, ray: &Ray, impact: &mut vector::Vector<f32>) -> bool {
        let tmp = self.global_to_local(ray.clone());
        let o = tmp.origin();
        let r = tmp.ray();

        let a = r[0] * r[0] + r[1] * r[1] + r[2] * r[2];
        let b = 2.0 * (r[0] * o[0] + r[1] * o[1] + r[2] * o[2]);
        let c = o[0] * o[0] + o[1] * o[1] + o[2] * o[2] - 1.0;
        let d = b * b - 4.0 * a * c;

        d >= 0.
    }
}
