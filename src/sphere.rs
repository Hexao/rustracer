use rulinalg::matrix::Matrix;
use rulinalg::vector::Vector;
use crate::object::Object;
use crate::ray::Ray;

pub struct Sphere {
    tra: Matrix<f32>,
    inv: Matrix<f32>,

    radius: f32,
}

impl Sphere {
    pub fn new(radius: f32) -> Self {
        Sphere {
            tra: Matrix::identity(4),
            inv: Matrix::identity(4),
            radius,
        }
    }
}

impl Object for Sphere {
    fn global_to_local(&self, vec: Vector<f32>) -> Vector<f32> {
        &self.inv * vec
    }

    fn local_to_global(&self, vec: Vector<f32>) -> Vector<f32> {
        &self.tra * vec
    }

    fn intersect(&self, ray: &Ray) -> bool {
        true
    }
}
