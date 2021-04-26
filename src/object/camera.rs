use crossbeam;
use std::sync::Arc;
use rulinalg::matrix::Matrix;
use rulinalg::vector::Vector;

use crate::scene::Scene;
use crate::math::ray::Ray;
use crate::object::Movable;
use crate::material::Color;

pub enum Focal {
    Perspective(f32),
    Orthographic(f32),
}

pub struct Camera {
    tra: Matrix<f32>,
    inv: Matrix<f32>,

    focal: Focal,
    x: usize,
    y: usize,
}

impl Camera {
    pub fn new(x: usize, y: usize, focal: Focal) -> Self {
        Camera {
            tra: Matrix::identity(4),
            inv: Matrix::identity(4),
            x, y, focal,
        }
    }

    /**
     * ### Brief
     * Allow to render the Scene **scene** in a file named **file_name**
     *
     * ### Params
     * **scene** The scene to render
     * **file_name** Target file
     */
    pub fn render_in(&self, scene: &Scene, file_name: &str, thread_count: usize) {
        let mut buf = Vec::with_capacity(self.x * self.y);
        let arc_self = Arc::from(self);
        let scene = Arc::from(scene);
        println!("render scene...");

        crossbeam::scope(|scope| {
            let mut threads = vec![];
            let step = arc_self.y / thread_count + 1;

            for thread_id in 0..thread_count {
                let start_row = thread_id * step;
                let stop_row = (thread_id + 1) * step;

                let scene = scene.clone();
                let arc_self = arc_self.clone();
                let mut buf = Vec::with_capacity(step * arc_self.x);

                threads.push(scope.spawn(move |_| {
                    for y in start_row..stop_row {
                        if y >= arc_self.y {
                            break;
                        }

                        for x in 0..arc_self.x {
                            let ray = arc_self.local_to_global_ray(arc_self.get_ray(x, y));
                            let mut impact = Vector::zeros(3);

                            buf.push(match scene.intersect(ray, &mut impact) {
                                Some(object) => {
                                    object.impact_color(&impact)
                                },
                                None => Color::new_gray(0),
                            });
                        }
                    }

                    buf
                }));
            }

            for thread in threads {
                let partial_data = thread.join().unwrap();
                for pix in partial_data {
                    buf.append(&mut pix.to_vec());
                }
            }
        }).unwrap();

        println!("scene rendered !");
        let name = format!("{}.png", file_name);
        image::save_buffer(name, buf.as_slice(), self.x as u32, self.y as u32, image::ColorType::RGB(8)).unwrap();
    }

    /// Allow to render the Scene **scene** in a file named image.png
    ///
    /// ### Params
    /// **scene** The scene to render
    pub fn render(&self, scene: &Scene, thread_count: usize) {
        self.render_in(scene, "image", thread_count);
    }

    fn get_ray(&self, x: usize, y: usize) -> Ray {
        match self.focal {
            Focal::Perspective(focal) => {
                let size = self.x.min(self.y);
                let px =  (x as f32 - self.x as f32 / 2.0) / size as f32;
                let py = -(y as f32 - self.y as f32 / 2.0) / size as f32;

                Ray::new(px, py, 0.0, px, py, focal).normalized()
            }
            Focal::Orthographic(focal) => {
                let size = self.x.min(self.y) / focal as usize;
                let px =  (x as f32 - self.x as f32 / 2.0) / size as f32;
                let py = -(y as f32 - self.y as f32 / 2.0) / size as f32;

                Ray::new(px, py, 0.0, 0.0, 0.0, 1.0)
            }
        }
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
