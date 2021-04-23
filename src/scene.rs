use rulinalg::vector::Vector;
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

    pub fn intersect(&self, ray: Ray) -> bool {
        for obj in self.objects.iter() {
            let mut impact = Vector::zeros(3);
            if obj.intersect(&ray, &mut impact) {
                return true
            }
        }

        false
    }
}

unsafe impl Send for Scene {}
unsafe impl Sync for Scene {}
