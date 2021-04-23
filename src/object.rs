use rulinalg::vector::Vector;
use crate::ray::Ray;

pub trait Object {
    fn global_to_local(&self, ray: Ray) -> Ray;
    fn local_to_global(&self, ray: Ray) -> Ray;

    fn move_global(&mut self, x: f32, y: f32, z: f32);
    fn rotate_x(&mut self, angle: f32);
    fn rotate_y(&mut self, angle: f32);
    fn rotate_z(&mut self, angle: f32);
    fn scale(&mut self, scale: f32);

    fn intersect(&self, ray: &Ray, impact: &mut Vector<f32>) -> bool;
}
