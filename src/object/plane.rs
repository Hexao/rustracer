use crate::material::{MatProvider, Material};
use crate::object::{Movable, Object};
use crate::math::ray::Ray;

use rulinalg::vector::Vector;
use rulinalg::matrix::Matrix;

pub struct Plane {
    tra: Matrix<f32>,
    inv: Matrix<f32>,

    mat: Box<dyn MatProvider>,
}

impl Plane {
    pub fn new(mat: Box<dyn MatProvider>) -> Self {
        Self {
            tra: Matrix::identity(4),
            inv: Matrix::identity(4),
            mat,
        }
    }
}

impl Movable for Plane {
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

impl Object for Plane {
    fn intersect(&self, ray: &Ray, impact: &mut Vector<f32>) -> bool {
        let ray = self.global_to_local_ray(ray.clone());

        let coef = -ray.origin()[2] / ray.vector()[2];
        *impact = self.local_to_global_point(
            ray.origin() + ray.vector() * coef
        );

        coef > 0.0
    }

    fn normal(&self, at: &Vector<f32>, observer: &Vector<f32>) -> Ray {
        let local_obs = self.global_to_local_point(observer.clone());

        self.local_to_global_ray(
            Ray::new(
                at[0], at[1], at[2],
                0.0, 0.0, local_obs[2]
            )
        ).normalized()
    }

    fn material_at(&self, impact: &Vector<f32>) -> Material {
        let local = self.global_to_local_point(impact.clone());

        let x = (if local[0] > 0.0 { 0.0 } else { 1.0 } + local[0] % 1.0).abs();
        let y = (if local[1] < 0.0 { 0.0 } else { 1.0 } - local[1] % 1.0).abs();

        self.mat.material(x, y)
    }
}
