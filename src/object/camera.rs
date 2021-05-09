use crate::object::{Movable, Object};
use crate::material::Color;
use crate::math::ray::Ray;
use crate::scene::Scene;

use rulinalg::matrix::Matrix;
use rulinalg::vector::Vector;
use std::sync::Arc;
use crossbeam;

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

    /**
     * ### Brief
     * Allow to render the Scene **scene** in a file named **file_name**
     *
     * ### Params
     * **scene** The scene to render
     * **file_name** Target file
     */
    pub fn render_in(&self, scene: &Scene, file_name: &str, depth: usize, thread_count: usize) {
        let mut buf = Vec::with_capacity(self.x * self.y);
        let arc_self = Arc::from(self);
        let scene = Arc::from(scene);
    
        let start = std::time::Instant::now();
        println!("render scene...");

        crossbeam::scope(|scope| {
            let mut threads = vec![];
            let step = arc_self.y / thread_count + 1;

            for thread_id in 0..thread_count {
                let start_row = thread_id * step;
                let stop_row = (thread_id + 1) * step;

                let scene = scene.clone();
                let arc_self = arc_self.clone();

                threads.push(scope.spawn(move |_| {
                    let offsets = [(-0.25, -0.25), (0.25, -0.25), (-0.25, 0.25), (0.25, 0.25)];
                    let mut buf = Vec::with_capacity(step * arc_self.x);

                    for y in start_row..stop_row {
                        if y >= arc_self.y {
                            break;
                        }

                        for x in 0..arc_self.x {
                            buf.push(if arc_self.flags & Camera::ANTI_ALIASING != 0 {
                                let mut sr = 0;
                                let mut sg = 0;
                                let mut sb = 0;

                                for (ox, oy) in &offsets {
                                    let ray = arc_self.local_to_global_ray(arc_self.get_ray(x as f32 + ox, y as f32 + oy));
                                    let mut impact = Vector::zeros(4);

                                    let Color { red, green, blue } = match scene.closer(&ray, &mut impact) {
                                        Some(object) => self.impact_color(&ray, object, &impact, &scene, depth),
                                        None => scene.background(),
                                    };

                                    sr += red as u16;
                                    sg += green as u16;
                                    sb += blue as u16;
                                }

                                Color::new((sr / 4) as u8, (sg / 4) as u8, (sb / 4) as u8)
                            } else {
                                let ray = arc_self.local_to_global_ray(arc_self.get_ray(x as f32, y as f32));
                                let mut impact = Vector::zeros(4);

                                match scene.closer(&ray, &mut impact) {
                                    Some(object) => self.impact_color(&ray, object, &impact, &scene, depth),
                                    None => scene.background(),
                                }
                            });
                        }
                    }

                    buf
                }));
            }

            for thread in threads {
                let partial_data = thread.join().unwrap();
                for pix in partial_data {
                    buf.extend_from_slice(&mut pix.to_vec());
                }
            }
        }).unwrap();

        let dur = start.elapsed().as_secs_f64();
        println!("scene rendered in {:.2} sec!", dur);

        std::fs::create_dir_all(std::path::Path::new(file_name).parent().unwrap()).unwrap();
        image::save_buffer(file_name, buf.as_slice(), self.x as u32, self.y as u32, image::ColorType::RGB(8)).unwrap();
    }

    fn get_ray(&self, x: f32, y: f32) -> Ray {
        match self.focal {
            Focal::Perspective(focal) => {
                let size = self.x.min(self.y);
                let px =  (x - self.x as f32 / 2.0) / size as f32;
                let py = -(y - self.y as f32 / 2.0) / size as f32;

                Ray::new(px, py, 0.0, px, py, focal).normalized()
            }
            Focal::Orthographic(focal) => {
                let size = self.x.min(self.y) as f32 / focal;
                let px =  (x - self.x as f32 / 2.0) / size;
                let py = -(y - self.y as f32 / 2.0) / size;

                Ray::new(px, py, 0.0, 0.0, 0.0, 1.0)
            }
        }
    }

    fn impact_color(&self, ray: &Ray, object: &Box<dyn Object>, impact: &Vector<f32>, scene: &Scene, depth: usize) -> Color {
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

            diffuse += material.diffuse * light.diffuse() * alpha;
            specular += material.specular * (normal.vector() * 2.0 * alpha - vec_light).dot(&-ray.vector()).powf(material.shininess) * light.specular() * alpha;
        }

        if depth > 0 {
            if material.alpha < 255 {
                let mut impact_refraction = Vector::zeros(4);
                let refraction_ray = object.refracted_ray(ray, impact);
                let object = scene.closer(&refraction_ray, &mut impact_refraction);

                let coef_refraction = material.alpha as f32 / 255.0;
                diffuse = diffuse * coef_refraction + match object {
                    None => scene.background(),
                    Some(object) => {
                        self.impact_color(&refraction_ray, object, &impact_refraction, scene, depth - 1)
                    }
                } * (1.0 - coef_refraction);
            }

            if material.reflection > 0 {
                let mut impact_reflection = Vector::zeros(4);
                let reflected_ray = object.reflected_ray(ray, impact);
                let object = scene.closer(&reflected_ray, &mut impact_reflection);

                let coef_reflection = material.reflection as f32 / 255.0;
                reflection = match object {
                    None => scene.background(),
                    Some(object) => {
                        self.impact_color(&reflected_ray, object, &impact_reflection, scene, depth - 1)
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
