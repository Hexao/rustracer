use crate::material::{MatProvider, Material};
use crate::object::{Movable, Object};
use crate::math::{
    point::Point,
    ray::Ray
};

use rulinalg::matrix::Matrix;
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
    fn intersect(&self, ray: &Ray) -> Option<Point> {
        let ray = self.global_to_local_ray(ray);
        let (origin, vector) = ray.consume();

        let a = vector.x * vector.x + vector.y * vector.y + vector.z * vector.z;
        let b = 2.0 * (vector.x * origin.x + vector.y * origin.y + vector.z * origin.z);
        let c = origin.x * origin.x + origin.y * origin.y + origin.z * origin.z - 1.0;
        let d = b * b - 4.0 * a * c;

        if d >= 0. {
            let d_sqrt = d.sqrt();
            let x1 = (-b - d_sqrt) / (2.0 * a);
            let x2 = (-b + d_sqrt) / (2.0 * a);

            let imp = if x1 < 0. && x2 < 0. {
                None
            } else if x1 < x2 && x1 >= 0. {
                Some(origin + vector * x1)
            } else {
                Some(origin + vector * x2)
            }?;

            Some(self.local_to_global_point(&imp))
        } else {
            None
        }
    }

    fn normal(&self, at: &Point, observer: &Point) -> Ray {
        let local = self.global_to_local_point(at);
        let observer = self.global_to_local_point(observer);

        let ray = if observer.norm() > 1.0 {
            Ray::new(local, local)
        } else {
            Ray::new(local, -local)
        };

        self.local_to_global_ray(&ray).normalized()
    }

    fn material_at(&self, impact: &Point) -> Material {
        let impact = self.global_to_local_point(impact);

        let x = impact.z.atan2(impact.x) / TAU + 0.5;
        let y = impact.y.acos() / PI;

        self.mat.material(x, y)
    }

    fn outter_normal(&self, impact: &Point) -> Point {
        let observer = Point::default();
        -self.normal(impact, &self.local_to_global_point(&observer)).vector()
    }

    fn coef_refraction(&self) -> f32 {
        self.coef_refraction
    }
}
