pub mod sphere;
pub mod camera;

use rulinalg::vector::Vector;
use rulinalg::matrix::Matrix;
use crate::math::ray::Ray;

pub trait Movable {
    fn global_to_local(&self, ray: Ray) -> Ray;
    fn local_to_global(&self, ray: Ray) -> Ray;
    fn transform(&mut self, transform: Matrix<f32>);

    fn move_global(&mut self, x: f32, y: f32, z: f32) {
        let mat = Matrix::new(4, 4, vec![
            1., 0., 0., x,
            0., 1., 0., y,
            0., 0., 1., z,
            0., 0., 0., 1.
        ]);
        self.transform(mat);
    }

    fn rotate_x(&mut self, angle: f32) {
        let angle = angle.to_radians();

        let mat = Matrix::new(4, 4, vec![
            1., 0., 0., 0.,
            0., angle.cos(), -angle.sin(), 0.,
            0., angle.sin(), angle.cos(), 0.,
            0., 0., 0., 1.
        ]);
        self.transform(mat);
    }

    fn rotate_y(&mut self, angle: f32) {
        let angle = angle.to_radians();

        let mat = Matrix::new(4, 4, vec![
            angle.cos(), 0., angle.sin(), 0.,
            0., 1., 0., 0.,
            -angle.sin(), 0., angle.cos(), 0.,
            0., 0., 0., 1.
        ]);
        self.transform(mat);
    }

    fn rotate_z(&mut self, angle: f32) {
        let angle = angle.to_radians();

        let mat = Matrix::new(4, 4, vec![
            angle.cos(), -angle.sin(), 0., 0.,
            angle.sin(), angle.cos(), 0., 0.,
            0., 0., 1., 0.,
            0., 0., 0., 1.
        ]);
        self.transform(mat);

    }

    fn scale(&mut self, scale: f32) {        
        let mat = Matrix::new(4, 4, vec![
            scale, 0., 0., 0.,
            0., scale, 0., 0.,
            0., 0., scale, 0.,
            0., 0., 0., 1.
        ]);
        self.transform(mat);
    }
}

pub trait Object: Movable {
    fn intersect(&self, ray: Ray, impact: &mut Vector<f32>) -> bool;
}
