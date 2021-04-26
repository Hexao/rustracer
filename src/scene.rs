use rulinalg::vector::Vector;
use rulinalg::norm::Euclidean;

use crate::object::Object;
use crate::math::ray::Ray;

pub struct Scene {
    objects: Vec<Box<dyn Object>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::default(),
        }
    }

    pub fn add_object(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }

    pub fn intersect(&self, ray: Ray, impact: &mut Vector<f32>) -> Option<&Box<dyn Object>> {
        let mut hit = None;
        let mut dist = f32::INFINITY;

        for obj in self.objects.iter() {
            let mut new_impact = Vector::zeros(3);
            if obj.intersect(ray.clone(), &mut new_impact) {
                let new_dist = (&new_impact - ray.origin()).norm(Euclidean);

                if new_dist < dist {
                    *impact = new_impact;
                    dist = new_dist;
                    hit = Some(obj);
                }
            }
        }

        hit
    }
}

unsafe impl Send for Scene {}
unsafe impl Sync for Scene {}
