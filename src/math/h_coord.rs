use crate::math::point::Point;

use rulinalg::matrix::{BaseMatrix, Matrix};
use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
pub struct HCoord {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl HCoord {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn into_vec(self) -> Point {
        Point { x: self.x, y: self.y, z: self.z }
    }

    pub fn into_pt(self) -> Point {
        Point { x: self.x / self.w, y: self.y / self.w, z: self.z / self.w }
    }
}


impl<T> Mul<HCoord> for &Matrix<T> where T: Mul<f32, Output = f32> + Add<f32, Output = f32> + Copy {
    type Output = HCoord;

    fn mul(self, rhs: HCoord) -> Self::Output {
        assert!(self.rows() == 4 && self.cols() == 4);
        let mut r = [0.0, 0.0, 0.0, 0.0];

        for (id, col) in self.row_iter().enumerate() {
            r[id] = col[0] * rhs.x + col[1] * rhs.y + col[2] * rhs.z + col[3] * rhs.w;
        }

        HCoord::new(r[0], r[1], r[2], r[3])
    }
}
