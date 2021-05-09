use crate::math::h_coord::HCoord;

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn into_vec4(&self) -> HCoord {
        HCoord::new(self.x, self.y, self.z, 0.0)
    }

    pub fn into_pt4(&self) -> HCoord {
        HCoord::new(self.x, self.y, self.z, 1.0)
    }

    pub fn norm(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalized(self) -> Self {
        self / self.norm()
    }

    pub fn dot(&self, rhs: &Point) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0.0, y: 0.0, z: 0.0 }
    }
}

use std::ops::{Add, Sub, Mul, Div, Neg};

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Add<Point> for &Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Add<&Point> for Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}



impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        Point { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Sub<Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Sub<&Point> for Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        Point { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl Div<f32> for Point {
    type Output = Point;

    fn div(self, rhs: f32) -> Self::Output {
        Point { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl Div<f32> for &Point {
    type Output = Point;

    fn div(self, rhs: f32) -> Self::Output {
        Point { x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl Mul<f32> for Point {
    type Output = Point;

    fn mul(self, rhs: f32) -> Self::Output {
        Point { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Mul<f32> for &Point {
    type Output = Point;

    fn mul(self, rhs: f32) -> Self::Output {
        Point { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl Neg for &Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point { x: -self.x, y: -self.y, z: -self.z }
    }
}
