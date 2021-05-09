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

    pub fn closer(&self, ray: &Ray, impact: &mut Point) -> Option<&Box<dyn Object>> {
        let mut hit = None;
        let mut dist = f32::INFINITY;

        for obj in self.objects.iter() {
            let mut new_impact = Point::default();

            if obj.intersect(&ray, &mut new_impact) {
                let new_dist = (&new_impact - ray.origin()).norm();

                if new_dist < dist {
                    *impact = new_impact;
                    dist = new_dist;
                    hit = Some(obj);
                }
            }
        }

        hit
    }

    pub fn light_filter(&self, point: &Point, light: &Box<dyn Light>) -> Color {
        let (mut origin, vector) = light.ray_to_light(point).consume();
        origin = origin + vector * 0.01;

        let ray = Ray::new(origin, vector);
        let mut impact = Point::default();
        let object = self.closer(&ray, &mut impact);

        match object {
            None => Color::new_gray(255),
            Some(object) => {
                let dist_light = light.distance(point);
                let dist_object = (impact - point).norm();

                if dist_light > dist_object {
                    let material = object.material_at(&impact);
                    let alpha_coef = material.alpha as f32 / 255.0;
                    let shadow = (Color::new_gray(255) * (1.0 - alpha_coef)) -
                        (Color::new_gray(255) - material.diffuse) * alpha_coef;
                    
                    let sqrt2 = 2.0f32.sqrt();
                    (shadow * sqrt2) * (self.light_filter(&impact, light) * sqrt2)
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
