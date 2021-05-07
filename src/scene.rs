use crate::material::Color;
use crate::object::{
    light::Light,
    Object,
};
use crate::math::ray::Ray;

use rulinalg::norm::Euclidean;
use rulinalg::vector::Vector;

pub struct Scene {
    objects: Vec<Box<dyn Object>>,
    lights: Vec<Box<dyn Light>>,

    background: Color,
    ambient: Color,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::default(),
            lights: Vec::default(),
            background: Color::SKY,
            ambient: Color::new_gray(127),
        }
    }

    pub fn new_custom(background: Color, ambient: Color) -> Self {
        Scene {
            objects: Vec::default(),
            lights: Vec::default(),
            background, ambient
        }
    }

    pub fn background(&self) -> Color {
        self.background
    }

    pub fn ambient(&self) -> Color {
        self.ambient
    }

    pub fn add_object(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: Box<dyn Light>) {
        self.lights.push(light);
    }

    pub fn closer(&self, ray: &Ray, impact: &mut Vector<f32>) -> Option<&Box<dyn Object>> {
        let mut hit = None;
        let mut dist = f32::INFINITY;

        for obj in self.objects.iter() {
            let mut new_impact = Vector::zeros(4);

            if obj.intersect(&ray, &mut new_impact) {
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

    pub fn lights(&self) -> &Vec<Box<dyn Light>> {
        &self.lights
    }
}

unsafe impl Send for Scene {}
unsafe impl Sync for Scene {}
