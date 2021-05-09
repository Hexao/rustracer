use crate::math::point::Point;
use crate::material::Color;
use crate::object::{
    light::Light,
    Movable,
};

use rulinalg::matrix::Matrix;

pub struct PointLight {
    tra: Matrix<f32>,
    inv: Matrix<f32>,

    diffuse: Color,
    specular: Color,
}

impl PointLight {
    pub fn new(diffuse: Color, specular: Color) -> Self {
        Self {
            tra: Matrix::identity(4),
            inv: Matrix::identity(4),
            diffuse, specular
        }
    }
}

impl Movable for PointLight {
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

impl Light for PointLight {
    fn illuminate(&self, _point: &Point) -> bool {
        true
    }

    fn diffuse(&self) -> Color {
        self.diffuse
    }

    fn specular(&self) -> Color {
        self.specular
    }
}
