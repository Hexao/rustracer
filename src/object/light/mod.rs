pub mod point_light;

use crate::material::Color;
use crate::object::Movable;
use crate::math::ray::Ray;

use rulinalg::norm::Euclidean;
use rulinalg::vector::Vector;

pub trait Light: Movable {
    fn vec_from_light(&self, point: &Vector<f32>) -> Vector<f32> {
        let vec = self.local_to_global_vector(&self.global_to_local_point(&point));
        let norm = vec.norm(Euclidean);
        vec / norm
    }

    fn vec_to_light(&self, point: &Vector<f32>) -> Vector<f32> {
        let vec = self.local_to_global_vector(&-self.global_to_local_point(&point));
        let norm = vec.norm(Euclidean);
        vec / norm
    }

    fn ray_from_light(&self, point: &Vector<f32>) -> Ray {
        let local = self.global_to_local_point(&point);
        let ray = Ray::new(
            0.0, 0.0, 0.0, 
            local[0], local[1], local[2]
        );

        self.local_to_global_ray(&ray).normalized()
    }

    fn ray_to_light(&self, point: &Vector<f32>) -> Ray {
        let local = self.global_to_local_point(&point);
        let ray = Ray::new(
            local[0], local[1], local[2],
            -local[0], -local[1], -local[2]
        );

        self.local_to_global_ray(&ray).normalized()
    }

    fn distance(&self, to: &Vector<f32>) -> f32 {
        let point = self.global_to_local_point(&to);
        point.norm(Euclidean)
    }

    fn illuminate(&self, point: &Vector<f32>) -> bool;

    fn diffuse(&self) -> Color;
    fn specular(&self) -> Color;
}
