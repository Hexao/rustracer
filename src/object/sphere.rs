use crate::material::{MatProvider, Material};
use crate::object::{Movable, Object};
use crate::math::ray::Ray;

use rulinalg::norm::Euclidean;
use rulinalg::matrix::Matrix;
use rulinalg::vector::Vector;
use std::f32::consts::TAU;
use std::f32::consts::PI;

pub struct Sphere {
    tra: Matrix<f32>,
    inv: Matrix<f32>,

    mat: Box<dyn MatProvider>,
    coef_refraction: f32,
}

impl Sphere {
    pub fn new(mat: Box<dyn MatProvider>, coef_refraction: f32) -> Self {
        Self {
            tra: Matrix::identity(4),
            inv: Matrix::identity(4),
            coef_refraction,
            mat,
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
    fn intersect(&self, ray: &Ray, impact: &mut Vector<f32>) -> bool {
        let ray = self.global_to_local_ray(ray.clone());
        let (origin, vector) = ray.consume();

        let a = vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2];
        let b = 2.0 * (vector[0] * origin[0] + vector[1] * origin[1] + vector[2] * origin[2]);
        let c = origin[0] * origin[0] + origin[1] * origin[1] + origin[2] * origin[2] - 1.0;
        let d = b * b - 4.0 * a * c;

        if d >= 0. {
            let d_sqrt = d.sqrt();
            let x1 = (-b - d_sqrt) / (2.0 * a);
            let x2 = (-b + d_sqrt) / (2.0 * a);

            let imp = if x1 < 0. && x2 < 0. {
                return false;
            } else if x1 < x2 && x1 >= 0. {
                origin + vector * x1
            } else {
                origin + vector * x2
            };

            *impact = self.local_to_global_point(imp);
            true
        } else {
            false
        }
    }

    fn normal(&self, at: &Vector<f32>, observer: &Vector<f32>) -> Ray {
        let local = self.global_to_local_point(at.clone());
        let mut observer = self.global_to_local_point(observer.clone());
        observer[3] = 0.0;

        let ray = if observer.norm(Euclidean) > 1.0 {
            Ray::new(
                local[0], local[1], local[2],
                local[0], local[1], local[2]
            )
        } else {
            Ray::new(
                local[0], local[1], local[2],
                -local[0], -local[1], -local[2]
            )
        };

        self.local_to_global_ray(ray).normalized()
    }

    fn material_at(&self, impact: &Vector<f32>) -> Material {
        let impact = self.global_to_local_point(impact.clone());

        let x = impact[2].atan2(impact[0]) / TAU + 0.5;
        let y = impact[1].acos() / PI;

        self.mat.material(x, y)
    }

    fn outter_normal(&self, impact: &Vector<f32>) -> Vector<f32> {
        let observer = Vector::new(vec![0.0, 0.0, 0.0, 1.0]);
        -self.normal(impact, &self.local_to_global_point(observer)).vector()
    }

    fn coef_refraction(&self) -> f32 {
        self.coef_refraction
    }
}
