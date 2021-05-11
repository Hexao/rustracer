use crate::material::Color;
use crate::object::{
    light::Light,
    Movable,
};
use crate::math::{
    point::Point,
    ray::Ray,
};

use rulinalg::matrix::Matrix;

pub struct DirectionalLight {
    tra: Matrix<f32>,
    inv: Matrix<f32>,

    diffuse: Color,
    specular: Color,
}

impl DirectionalLight {
    pub fn new(diffuse: Color, specular: Color) -> Self {
        Self {
            tra: Matrix::identity(4),
            inv: Matrix::identity(4),
            diffuse, specular
        }
    }
}

impl Movable for DirectionalLight {
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

impl Light for DirectionalLight {
    fn illuminate(&self, _point: &Point) -> bool {
        true
    }

    fn diffuse(&self) -> Color {
        self.diffuse
    }

    fn specular(&self) -> Color {
        self.specular
    }

    fn vec_from_light(&self, _point: &Point) -> Point {
        let vec = self.local_to_global_vector(&Point::new(0.0, 0.0, 1.0));
        vec.normalized()
    }

    fn vec_to_light(&self, _point: &Point) -> Point {
        let vec = self.local_to_global_vector(&Point::new(0.0, 0.0, -1.0));
        vec.normalized()
    }

    fn ray_from_light(&self, point: &Point) -> Ray {
        let local = self.global_to_local_point(point);
        let ray = Ray::new(
            Point::new(local.x, local.y, -f32::INFINITY),
            Point::new(0.0, 0.0, 1.0)
        );

        self.local_to_global_ray(&ray).normalized()
    }

    fn ray_to_light(&self, point: &Point) -> Ray {
        let local = self.global_to_local_point(point);
        let ray = Ray::new(local, Point::new(0.0, 0.0, -1.0));

        self.local_to_global_ray(&ray).normalized()
    }

    fn distance(&self, _to: &Point) -> f32 {
        f32::INFINITY
    }
}
