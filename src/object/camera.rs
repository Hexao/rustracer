use crate::object::{Movable, Object};
use crate::math::point::Point;
use crate::material::Color;
use crate::math::ray::Ray;
use crate::scene::Scene;

use serde::{Deserialize, Deserializer, de::{Visitor, Error, MapAccess}};
use rulinalg::matrix::Matrix;

pub enum Focal {
    Perspective(f32),
    Orthographic(f32),
}

pub struct Camera {
    tra: Matrix<f32>,
    inv: Matrix<f32>,

    focal: Focal,
    flags: u8,
    x: usize,
    y: usize,
}

impl Camera {
    pub const ANTI_ALIASING: u8 = 1;
    pub const NO_SHADOW    : u8 = 2;

    pub fn new(x: usize, y: usize, focal: Focal) -> Self {
        Camera {
            tra: Matrix::identity(4),
            inv: Matrix::identity(4),
            flags: 0, x, y, focal,
        }
    }

    pub fn set_flags(&mut self, flags: u8) {
        self.flags = flags;
    }

    /// ### Brief
    /// Allow to render the Scene **scene** in a file named **file_name**
    ///
    /// ### Params
    /// **scene** The scene to render
    /// **file_name** Target file
    pub fn render_in(&self, scene: &Scene, file_name: &str, depth: usize, thread_count: usize) {
        let mut buf = Vec::with_capacity(self.x * self.y * 3);

        let start = std::time::Instant::now();
        println!("render scene...");

        std::thread::scope(|scope| {
            let mod_zero = (self.y % thread_count).min(1);
            let step = self.y / thread_count + mod_zero;
            let mut threads = vec![];

            for thread_id in 0..thread_count {
                let start_row = thread_id * step;
                let stop_row = (thread_id + 1) * step;

                threads.push(if self.flags & Camera::ANTI_ALIASING != 0 {
                    scope.spawn(move || {
                        let offsets = [(-0.25, -0.25), (0.25, -0.25), (-0.25, 0.25), (0.25, 0.25)];
                        let mut buf = Vec::with_capacity(step * self.x);

                        for y in (start_row..stop_row.min(self.y)).map(|y| y as f32) {
                            for x in (0..self.x).map(|x| x as f32) {
                                buf.push({
                                    let mut avg = Color::default();

                                    for (ox, oy) in &offsets {
                                        let ray = self.local_to_global_ray(&self.get_ray(x + ox, y + oy));

                                        avg += match scene.closer(&ray) {
                                            Some((object, impact)) => self.impact_color(&ray, object, &impact, scene, depth),
                                            None => scene.background(),
                                        } * 0.25;
                                    }

                                    avg
                                });
                            }
                        }

                        buf
                    })
                } else {
                    scope.spawn(move || {
                        let mut buf = Vec::with_capacity(step * self.x);

                        for y in (start_row..stop_row.min(self.y)).map(|y| y as f32) {
                            for x in (0..self.x).map(|x| x as f32) {
                                buf.push({
                                    let ray = self.local_to_global_ray(&self.get_ray(x, y));

                                    match scene.closer(&ray) {
                                        Some((object, impact)) => self.impact_color(&ray, object, &impact, scene, depth),
                                        None => scene.background(),
                                    }
                                });
                            }
                        }

                        buf
                    })
                });
            }

            for thread in threads {
                let partial_data = thread.join().unwrap();

                buf.extend(
                    partial_data.into_iter().flat_map(|pix| pix.to_array())
                );
            }
        });

        let dur = start.elapsed().as_secs_f32();
        println!("scene rendered in {dur:.2} sec!");

        std::fs::create_dir_all(std::path::Path::new(file_name).parent().unwrap()).unwrap();
        image::save_buffer(file_name, buf.as_slice(), self.x as u32, self.y as u32, image::ColorType::Rgb8).unwrap();
    }

    fn get_ray(&self, x: f32, y: f32) -> Ray {
        match self.focal {
            Focal::Perspective(focal) => {
                let size = self.x.min(self.y) as f32;
                let px =  (x - self.x as f32 / 2.0) / size;
                let py = -(y - self.y as f32 / 2.0) / size;

                let origin = Point::new(px, py, 0.0);
                let vector = Point::new(px, py, focal);

                Ray::new(origin, vector).normalized()
            }
            Focal::Orthographic(focal) => {
                let size = self.x.min(self.y) as f32 / focal;
                let px =  (x - self.x as f32 / 2.0) / size;
                let py = -(y - self.y as f32 / 2.0) / size;

                let origin = Point::new(px, py, 0.0);
                let vector = Point::new(0.0, 0.0, 1.0);

                Ray::new(origin, vector).normalized()
            }
        }
    }

    fn impact_color(&self, ray: &Ray, object: &dyn Object, impact: &Point, scene: &Scene, depth: usize) -> Color {
        let mut specular = Color::default();
        let mut reflection = Color::default();
        let material = object.material_at(impact);
        let mut diffuse = material.ambient * scene.ambient();
        let normal = object.normal(impact, ray.origin());

        for light in scene.lights() {
            if !light.illuminate(impact) {
                continue;
            }

            let vec_light = light.vec_to_light(impact);
            let alpha = vec_light.dot(normal.vector());
            if alpha <= 0.0 {
                continue;
            }

            let shadow = if (self.flags & Camera::NO_SHADOW) != 0 {
                Color::new_gray(255)
            } else {
                scene.light_filter(impact, light.as_ref(), 0)
            };

            diffuse += material.diffuse * light.diffuse() * alpha * shadow;
            specular += material.specular * (normal.vector() * 2.0 * alpha - vec_light).dot(&-ray.vector()).powf(material.shininess) * light.specular() * alpha * shadow;
        }

        if depth > 0 {
            if material.alpha < 255 {
                let refraction_ray = object.refracted_ray(ray, impact);
                let closer = scene.closer(&refraction_ray);

                let coef_refraction = material.alpha as f32 / 255.0;
                diffuse = diffuse * coef_refraction + match closer {
                    None => scene.background(),
                    Some((object, impact)) => {
                        self.impact_color(&refraction_ray, object, &impact, scene, depth - 1)
                    }
                } * (1.0 - coef_refraction);
            }

            if material.reflection > 0 {
                let reflected_ray = object.reflected_ray(ray, impact);
                let closer = scene.closer(&reflected_ray);

                let coef_reflection = material.reflection as f32 / 255.0;
                reflection = match closer {
                    None => scene.background(),
                    Some((object, impact)) => {
                        self.impact_color(&reflected_ray, object, &impact, scene, depth - 1)
                    }
                } * coef_reflection;
                diffuse = diffuse * (1.0 - coef_reflection);
            }
        }

        diffuse + specular + reflection
    }
}

impl Movable for Camera {
    fn tra(&self) -> &Matrix<f32> {
        &self.tra
    }

    fn tra_mut(&mut self) -> &mut Matrix<f32> {
        &mut self.tra
    }

    fn inv(&self) -> &Matrix<f32> {
        &self.inv
    }

    fn inv_mut(&mut self) -> &mut Matrix<f32> {
        &mut self.inv
    }
}

impl<'de> Deserialize<'de> for Focal {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        const FOCAL: &[&str] = &["PERSPECTIVE", "ORTHOGRAPHIC"];
        const FIELDS: &[&str] = &["type", "size"];
        struct FocalVisitor;

        impl<'de> Visitor<'de> for FocalVisitor {
            type Value = Focal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Focal struct")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
                let mut focal_type = None;
                let mut size = None;

                while let Some(field) = map.next_key()? {
                    match field {
                        "type" => focal_type = Some(map.next_value()?),
                        "size" => size = Some(map.next_value()?),
                        _ => return Err(Error::unknown_field(field, FIELDS)),
                    }
                }

                let focal_type = focal_type.ok_or_else(|| Error::missing_field("type"))?;

                match focal_type {
                    "PERSPECTIVE" => {
                        let size = size.unwrap_or(1.7);
                        Ok(Focal::Perspective(size))
                    }
                    "ORTHOGRAPHIC" => {
                        let size = size.unwrap_or(1.0);
                        Ok(Focal::Orthographic(size))
                    }
                    _ => Err(Error::unknown_variant(focal_type, FOCAL)),
                }
            }
        }

        deserializer.deserialize_map(FocalVisitor)
    }
}

impl<'de> Deserialize<'de> for Camera {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        const FIELDS: &[&str] = &["size", "focal", "flags", "transform", "rotate", "scale"];
        const FLAGS: &[&str] = &["ANTI_ALIASING", "NO_SHADOW"];
        struct CameraVisitor;

        impl<'de> Visitor<'de> for CameraVisitor {
            type Value = Camera;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Camera struct")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error> where A: MapAccess<'de> {
                let mut size: Option<[usize; 2]> = None;
                let mut focal = None;
                let mut flags: Option<Vec<&str>> = None;
                let mut transform = None;
                let mut rotate = None;
                let mut scale = None;

                while let Some(field) = map.next_key()? {
                    match field {
                        "size" => size = Some(map.next_value()?),
                        "focal" => focal = Some(map.next_value()?),
                        "flags" => flags = Some(map.next_value()?),
                        "transform" => transform = Some(map.next_value()?),
                        "rotate" => rotate = Some(map.next_value()?),
                        "scale" => scale = Some(map.next_value()?),
                        _ => return Err(Error::unknown_field(field, FIELDS)),
                    }
                }

                let [x, y] = size.unwrap_or([1920, 1080]);
                let focal = focal.unwrap_or(Focal::Perspective(1.7));

                let flags = match flags {
                    None => 0,
                    Some(flags) => {
                        let mut flag = 0;

                        for entree in flags {
                            match entree {
                                "ANTI_ALIASING" => flag |= Camera::ANTI_ALIASING,
                                "NO_SHADOW" => flag |= Camera::NO_SHADOW,
                                _ => return Err(Error::unknown_variant(entree, FLAGS)),
                            }
                        }

                        flag
                    }
                };

                let mut camera = Camera::new(x, y, focal);
                camera.set_flags(flags);

                if let Some(Point {x, y , z}) = transform {
                    camera.move_global(x, y, z);
                }

                if let Some(Point {x, y , z}) = rotate {
                    camera.rotate_x(x);
                    camera.rotate_y(y);
                    camera.rotate_z(z);
                }

                if let Some(scale) = scale {
                    camera.scale(scale);
                }

                Ok(camera)
            }
        }

        deserializer.deserialize_map(CameraVisitor)
    }
}
