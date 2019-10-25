use rulinalg::matrix::Matrix;
use crate::scene::Scene;
use crate::ray::Ray;

pub struct Camera {
    tra: Matrix<f32>,
    inv: Matrix<f32>,

    focal: f32,
    size: u16,
    x: u16,
    y: u16,
}

impl Camera {
    pub fn new(x: u16, y: u16, focal: f32) -> Self {
        Camera {
            size: if x < y {x} else {y},
            tra: Matrix::identity(4),
            inv: Matrix::identity(4),
            x, y, focal,
        }
    }

    pub fn render(&self, scene: &Scene) {
        println!("render scene...");

        let r = self.get_ray(0, 0);
    }

    fn get_ray(&self, x: u16, y: u16) -> Ray {
        let px = (x as f32 - self.x as f32 /  2.0) / self.size as f32;
        let py = (y as f32 - self.y as f32 / -2.0) / self.size as f32; 

        println!("[{}, {}]", px, py);
        Ray::new(px, py, 0.0, px, py, -self.focal).normalized()
    }
}
