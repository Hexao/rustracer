use crate::material::{MatProvider, Material};
use crate::object::{Movable, Object};
use crate::math::{
    point::Point,
    ray::Ray
};

use rulinalg::matrix::Matrix;

pub struct Plane {
    tra: Matrix<f32>,
    inv: Matrix<f32>,

    mat: Box<dyn MatProvider>,
    coef_refraction: f32,
}

impl Plane {
    pub fn new(mat: Box<dyn MatProvider>, coef_refraction: f32) -> Self {
        Self {
            tra: Matrix::identity(4),
            inv: Matrix::identity(4),
            coef_refraction,
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
    fn intersect(&self, ray: &Ray) -> Option<Point> {
        let ray = self.global_to_local_ray(ray);
        let coef = -ray.origin().z / ray.vector().z;

        if coef > 0.0 {
            Some(self.local_to_global_point(
                &(ray.origin() + ray.vector() * coef)
            ))
        } else {
            None
        }
    }

    fn normal(&self, at: &Point, observer: &Point) -> Ray {
        let local_obs = self.global_to_local_point(observer);

        self.local_to_global_ray(
            &Ray::new(*at, Point::new(0.0, 0.0, local_obs.z))
        ).normalized()
    }

    fn material_at(&self, impact: &Point) -> Material {
        let local = self.global_to_local_point(impact);

        let x = (if local.x > 0.0 { 0.0 } else { 1.0 } + local.x % 1.0).abs();
        let y = (if local.y < 0.0 { 0.0 } else { 1.0 } - local.y % 1.0).abs();

        self.mat.material(x, y)
    }

    fn outter_normal(&self, impact: &Point) -> Point {
        let observer = Point::new(0.0, 0.0, 1.0);
        let (_origin, vector) = self.normal(impact, &self.local_to_global_point(&observer)).consume();
        vector
    }

    fn coef_refraction(&self) -> f32 {
        self.coef_refraction
    }
}
