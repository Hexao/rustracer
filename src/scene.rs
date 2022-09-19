use crate::material::Color;
use crate::object::{
    light::Light,
    Object,
};
use crate::math::{
    point::Point,
    ray::Ray,
};

pub struct Scene {
    objects: Vec<Box<dyn Object>>,
    lights: Vec<Box<dyn Light>>,

    background: Color,
    ambient: Color,
}

impl Scene {
    pub fn new(objects: Vec<Box<dyn Object>>, lights: Vec<Box<dyn Light>>, background: Color, ambient: Color) -> Self {
        Scene { objects, lights, background, ambient }
    }

    pub fn background(&self) -> Color {
        self.background
    }

    pub fn ambient(&self) -> Color {
        self.ambient
    }

    pub fn closer(&self, ray: &Ray) -> Option<(&dyn Object, Point)> {
        let mut hit = None;
        let mut dist = f32::INFINITY;

        for obj in self.objects.iter() {
            if let Some(impact) = obj.intersect(ray) {
                let new_dist = (impact - ray.origin()).norm();

                if new_dist < dist {
                    dist = new_dist;
                    hit = Some((obj.as_ref(), impact));
                }
            }
        }

        hit
    }

    pub fn light_filter(&self, point: &Point, light: &dyn Light, depth: usize) -> Color {
        let (mut origin, vector) = light.ray_to_light(point).consume();
        origin = origin + vector * 0.01;

        let ray = Ray::new(origin, vector);
        let closer = self.closer(&ray);

        match closer {
            None => Color::new_gray(255),
            Some((object, impact)) => {
                if point == &impact {
                    println!("recursive spot at depth {}", depth);
                    return Color::new_gray(255);
                }

                let dist_light = light.distance(point);
                let dist_object = (impact - point).norm();

                if dist_light > dist_object {
                    let material = object.material_at(&impact);
                    let alpha_coef = material.alpha as f32 / 255.0;
                    let shadow = Color::new_gray(255) * (1.0 - alpha_coef) -
                        (Color::new_gray(255) - material.diffuse) * alpha_coef;

                    use std::f32::consts::SQRT_2;
                    (shadow * SQRT_2) * (self.light_filter(&impact, light, depth + 1) * SQRT_2)
                } else {
                    Color::new_gray(255)
                }
            }
        }
    }

    pub fn lights(&self) -> &Vec<Box<dyn Light>> {
        &self.lights
    }
}

unsafe impl Send for Scene {}
unsafe impl Sync for Scene {}
