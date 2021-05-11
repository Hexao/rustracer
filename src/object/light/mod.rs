pub mod directional_light;
pub mod point_light;

use crate::material::Color;
use crate::object::Movable;
use crate::math::{
    point::Point,
    ray::Ray
};

pub trait Light: Movable {
    fn vec_from_light(&self, point: &Point) -> Point {
        let vec = self.local_to_global_vector(&self.global_to_local_point(point));
        vec.normalized()
    }

    fn vec_to_light(&self, point: &Point) -> Point {
        let vec = self.local_to_global_vector(&-self.global_to_local_point(point));
        vec.normalized()
    }

    fn ray_from_light(&self, point: &Point) -> Ray {
        let local = self.global_to_local_point(point);
        let ray = Ray::new(Point::default(), local);

        self.local_to_global_ray(&ray).normalized()
    }

    fn ray_to_light(&self, point: &Point) -> Ray {
        let local = self.global_to_local_point(point);
        let ray = Ray::new(local, -local);

        self.local_to_global_ray(&ray).normalized()
    }

    fn distance(&self, to: &Point) -> f32 {
        let point = self.global_to_local_point(to);
        point.norm()
    }

    fn illuminate(&self, point: &Point) -> bool;

    fn diffuse(&self) -> Color;
    fn specular(&self) -> Color;
}
