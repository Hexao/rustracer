#![allow(dead_code, unused_imports, unused_variables)]

mod camera;
mod object;
mod ray;
mod scene;
mod sphere;

use camera::{Camera, Focal};
use object::Object;
use scene::Scene;

fn main() {
    // let c = Camera::new(192, 108, 0.4);
    let c = Camera::new(1920, 1080, Focal::Perspective(1.2));
    let mut s = Scene::new();

    let mut sphere: Box<dyn Object> = Box::new(sphere::Sphere::new());
    sphere.move_global(0.0, 0.0, 15.0);
    s.add_object(sphere);

    let mut sphere: Box<dyn Object> = Box::new(sphere::Sphere::new());
    sphere.move_global(2.5, 0.0, 15.0);
    s.add_object(sphere);

    let mut sphere: Box<dyn Object> = Box::new(sphere::Sphere::new());
    sphere.move_global(5.0, 0.0, 15.0);
    s.add_object(sphere);

    let mut sphere: Box<dyn Object> = Box::new(sphere::Sphere::new());
    sphere.move_global(7.5, 0.0, 15.0);
    s.add_object(sphere);

    let mut sphere: Box<dyn Object> = Box::new(sphere::Sphere::new());
    sphere.move_global(10.0, 0.0, 15.0);
    s.add_object(sphere);

    let mut sphere: Box<dyn Object> = Box::new(sphere::Sphere::new());
    sphere.move_global(12.5, 0.0, 15.0);
    s.add_object(sphere);

    let mut sphere: Box<dyn Object> = Box::new(sphere::Sphere::new());
    sphere.move_global(15.0, 0.0, 15.0);
    s.add_object(sphere);

    let start = std::time::Instant::now();
    c.render(&s, 1);

    let dur = start.elapsed().as_secs_f64();
    println!("rendered in {:.2} sec", dur);
}
