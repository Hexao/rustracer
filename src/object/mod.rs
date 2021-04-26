pub mod sphere;
pub mod camera;

use rulinalg::vector::Vector;
use rulinalg::matrix::Matrix;
use rulinalg::vector;

use crate::math::ray::Ray;

pub trait Movable {
    fn tra(&self) -> &Matrix<f32>;
    fn tra_mut(&mut self) -> &mut Matrix<f32>;

    fn inv(&self) -> &Matrix<f32>;
    fn inv_mut(&mut self) -> &mut Matrix<f32>;

    fn local_to_global_ray(&self, ray: Ray) -> Ray {
        let origin = self.local_to_global_point(ray.origin());
        let vector = self.local_to_global_vector(ray.vector());

        Ray::new(origin[0], origin[1], origin[2], vector[0], vector[1], vector[2])
    }

    fn local_to_global_point(&self, pts: Vector<f32>) -> Vector<f32> {
        let mut pts = vector![pts[0], pts[1], pts[2], 1.0];
        pts = self.tra() * pts;

        vector![
            pts[0] / pts[3],
            pts[1] / pts[3],
            pts[2] / pts[3]
        ]
    }

    fn local_to_global_vector(&self, vec: Vector<f32>) -> Vector<f32> {
        let mut vec = vector![vec[0], vec[1], vec[2], 0.0];
        vec = self.tra() * vec;

        vector![vec[0], vec[1], vec[2]]
    }

    fn global_to_local_ray(&self, ray: Ray) -> Ray {
        let origin = self.global_to_local_point(ray.origin());
        let vector = self.global_to_local_vector(ray.vector());

        Ray::new(origin[0], origin[1], origin[2], vector[0], vector[1], vector[2])
    }

    fn global_to_local_point(&self, pts: Vector<f32>) -> Vector<f32> {
        let mut pts = vector![pts[0], pts[1], pts[2], 1.0];
        pts = self.inv() * pts;

        vector![
            pts[0] / pts[3],
            pts[1] / pts[3],
            pts[2] / pts[3]
        ]
    }

    fn global_to_local_vector(&self, vec: Vector<f32>) -> Vector<f32> {
        let mut vec = vector![vec[0], vec[1], vec[2], 0.0];
        vec = self.inv() * vec;

        vector![vec[0], vec[1], vec[2]]
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
        
        *self.tra_mut() = mat * self.tra();
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
        
        *self.tra_mut() = mat * self.tra();
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
        
        *self.tra_mut() = mat * self.tra();
        *self.inv_mut() = self.tra().clone().inverse().unwrap();
    }

    fn scale(&mut self, scale: f32) {        
        let mat = Matrix::new(4, 4, vec![
            scale, 0., 0., 0.,
            0., scale, 0., 0.,
            0., 0., scale, 0.,
            0., 0., 0., 1.
        ]);
        
        *self.tra_mut() = mat * self.tra();
        *self.inv_mut() = self.tra().clone().inverse().unwrap();
    }
}

pub trait Object: Movable {
    fn intersect(&self, ray: Ray, impact: &mut Vector<f32>) -> bool;
}
