use crate::math::point::Point;

#[derive(Clone, Copy)]
pub struct Ray {
    origin: Point,
    vector: Point,
}

impl Ray {
    pub fn new(origin: Point, vector: Point) -> Self {
        Self { origin, vector }
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn vector(&self) -> &Point {
        &self.vector
    }

    pub fn normalized(self) -> Self {
        let ray_norm = self.vector.norm();

        Ray {
            origin: self.origin,
            vector: self.vector / ray_norm,
        }
    }

    pub fn consume(self) -> (Point, Point) {
        (self.origin, self.vector)
    }
}
