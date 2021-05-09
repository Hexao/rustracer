pub mod sphere;
pub mod camera;
pub mod plane;
pub mod light;

use crate::material::Material;
use crate::math::{
    point::Point,
    ray::Ray
};

use rulinalg::matrix::Matrix;

pub trait Movable {
    fn tra(&self) -> &Matrix<f32>;
    fn tra_mut(&mut self) -> &mut Matrix<f32>;

    fn inv(&self) -> &Matrix<f32>;
    fn inv_mut(&mut self) -> &mut Matrix<f32>;

    fn local_to_global_ray(&self, ray: &Ray) -> Ray {
        let origin = self.local_to_global_point(ray.origin());
        let vector = self.local_to_global_vector(ray.vector());
        Ray::new(origin, vector)
    }

    fn local_to_global_point(&self, pts: &Point) -> Point {
        let pts = pts.into_pt4();
        let pts = self.tra() * pts;
        pts.into_pt()
    }

    fn local_to_global_vector(&self, vec: &Point) -> Point {
        let vec = vec.into_vec4();
        let vec = self.tra() * vec;
        vec.into_vec()
    }

    fn global_to_local_ray(&self, ray: &Ray) -> Ray {
        let origin = self.global_to_local_point(ray.origin());
        let vector = self.global_to_local_vector(ray.vector());
        Ray::new(origin, vector)
    }

    fn global_to_local_point(&self, pts: &Point) -> Point {
        let pts = pts.into_pt4();
        let pts = self.inv() * pts;
        pts.into_pt()
    }

    fn global_to_local_vector(&self, vec: &Point) -> Point {
        let vec = vec.into_vec4();
        let vec = self.inv() * vec;
        vec.into_vec()
    }

    fn move_global(&mut self, x: f32, y: f32, z: f32) {
        let mat = Matrix::new(4, 4, vec![
            1., 0., 0., x,
            0., 1., 0., y,
            0., 0., 1., z,
            0., 0., 0., 1.
        ]);

        *self.tra_mut() = mat * self.tra();
        *self.inv_mut() = self.tra().clone().inverse().unwrap();
    }

    fn rotate_x(&mut self, angle: f32) {
        let angle = angle.to_radians();

        let mat = Matrix::new(4, 4, vec![
            1., 0., 0., 0.,
            0., angle.cos(), -angle.sin(), 0.,
            0., angle.sin(), angle.cos(), 0.,
            0., 0., 0., 1.
        ]);

        *self.tra_mut() = self.tra() * mat;
        *self.inv_mut() = self.tra().clone().inverse().unwrap();
    }

    fn rotate_y(&mut self, angle: f32) {
        let angle = angle.to_radians();

        let mat = Matrix::new(4, 4, vec![
            angle.cos(), 0., angle.sin(), 0.,
            0., 1., 0., 0.,
            -angle.sin(), 0., angle.cos(), 0.,
            0., 0., 0., 1.
        ]);

        *self.tra_mut() = self.tra() * mat;
        *self.inv_mut() = self.tra().clone().inverse().unwrap();
    }

    fn rotate_z(&mut self, angle: f32) {
        let angle = angle.to_radians();

        let mat = Matrix::new(4, 4, vec![
            angle.cos(), -angle.sin(), 0., 0.,
            angle.sin(), angle.cos(), 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.
        ]);

        *self.tra_mut() = self.tra() * mat;
        *self.inv_mut() = self.tra().clone().inverse().unwrap();
    }

    fn scale(&mut self, scale: f32) {
        let mat = Matrix::new(4, 4, vec![
            scale, 0., 0., 0.,
            0., scale, 0., 0.,
            0., 0., scale, 0.,
            0., 0., 0., 1.
        ]);

        *self.tra_mut() = self.tra() * mat;
        *self.inv_mut() = self.tra().clone().inverse().unwrap();
    }
}

pub trait Object: Movable {
    fn intersect(&self, ray: &Ray, impact: &mut Point) -> bool;
    fn normal(&self, at: &Point, observer: &Point) -> Ray;
    fn material_at(&self, impact: &Point) -> Material;
    fn outter_normal(&self, impact: &Point) -> Point;
    fn coef_refraction(&self) -> f32;

    fn reflected_ray(&self, ray: &Ray, impact: &Point) -> Ray {
        let normal = self.normal(impact, ray.origin());

        let gap = 0.0005;
        let dot = ray.vector().dot(normal.vector());
        let reflected = ray.vector() - normal.vector() * 2.0 * dot;

        Ray::new(impact + reflected * gap, reflected)
    }

    fn refracted_ray(&self, ray: &Ray, impact: &Point) -> Ray {
        let mut normal = self.outter_normal(impact);

        let mut cosi = ray.vector().dot(&normal);
        let eta = if cosi < 0.0 {
            cosi = -cosi;
            1.0 / self.coef_refraction()
        } else {
            normal = -normal;
            self.coef_refraction() / 1.0
        };

        let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
        let refracted = if k < 0.0 {
            Point::default()
        } else {
            ray.vector() * eta + normal * (eta * cosi - k.sqrt())
        };

        let gap = 0.0005;
        Ray::new(impact + refracted * gap, refracted)
    }
}
