pub mod sphere;
pub mod camera;
pub mod plane;
pub mod light;

use crate::material::Material;
use crate::math::ray::Ray;

use rulinalg::vector::Vector;
use rulinalg::matrix::Matrix;

pub trait Movable {
    fn tra(&self) -> &Matrix<f32>;
    fn tra_mut(&mut self) -> &mut Matrix<f32>;

    fn inv(&self) -> &Matrix<f32>;
    fn inv_mut(&mut self) -> &mut Matrix<f32>;

    fn local_to_global_ray(&self, ray: Ray) -> Ray {
        let (o, v) = ray.consume();
        let origin = self.local_to_global_point(o);
        let vector = self.local_to_global_vector(v);

        Ray::new(origin[0], origin[1], origin[2], vector[0], vector[1], vector[2])
    }

    fn local_to_global_point(&self, mut pts: Vector<f32>) -> Vector<f32> {
        pts[3] = 1.0;
        pts = self.tra() * pts;
        &pts / pts[3]
    }

    fn local_to_global_vector(&self, mut vec: Vector<f32>) -> Vector<f32> {
        vec[3] = 0.0;
        self.tra() * vec
    }

    fn global_to_local_ray(&self, ray: Ray) -> Ray {
        let (o, v) = ray.consume();
        let origin = self.global_to_local_point(o);
        let vector = self.global_to_local_vector(v);

        Ray::new(origin[0], origin[1], origin[2], vector[0], vector[1], vector[2])
    }

    fn global_to_local_point(&self, mut pts: Vector<f32>) -> Vector<f32> {
        pts[3] = 1.0;
        pts = self.inv() * pts;
        &pts / pts[3]
    }

    fn global_to_local_vector(&self, mut vec: Vector<f32>) -> Vector<f32> {
        vec[3] = 0.0;
        self.inv() * vec
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
    fn intersect(&self, ray: &Ray, impact: &mut Vector<f32>) -> bool;
    fn normal(&self, at: &Vector<f32>, observer: &Vector<f32>) -> Ray;
    fn material_at(&self, impact: &Vector<f32>) -> Material;
}
