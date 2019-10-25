use rulinalg::vector::Vector;
use crate::ray::Ray;

pub trait Object {
    fn global_to_local(&self, vec: Vector<f32>) -> Vector<f32>;
    fn local_to_global(&self, vec: Vector<f32>) -> Vector<f32>;

    fn intersect(&self, ray: &Ray) -> bool;
}
