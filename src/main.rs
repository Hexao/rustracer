pub mod scene;
pub mod camera;

pub mod math;
pub mod object;

use object::{Object, sphere::Sphere};
use camera::{Camera, Focal};
use scene::Scene;

fn main() {
    // let c = Camera::new(192, 108, 0.4);
    let c = Camera::new(1920, 1080, Focal::Orthographic(1.5));
    let mut s = Scene::new();

    let mut sphere: Box<dyn Object> = Box::new(Sphere::new());
    sphere.move_global(0.0, 0.0, 15.0);
    s.add_object(sphere);

    let mut sphere: Box<dyn Object> = Box::new(Sphere::new());
    sphere.move_global(2.5, 0.0, 15.0);
    s.add_object(sphere);

    let mut sphere: Box<dyn Object> = Box::new(Sphere::new());
    sphere.move_global(5.0, 0.0, 15.0);
    s.add_object(sphere);

    let mut sphere: Box<dyn Object> = Box::new(Sphere::new());
    sphere.move_global(7.5, 0.0, 15.0);
    s.add_object(sphere);

    let mut sphere: Box<dyn Object> = Box::new(Sphere::new());
    sphere.move_global(10.0, 0.0, 15.0);
    s.add_object(sphere);

    let mut sphere: Box<dyn Object> = Box::new(Sphere::new());
    sphere.move_global(12.5, 0.0, 15.0);
    s.add_object(sphere);

    let mut sphere: Box<dyn Object> = Box::new(Sphere::new());
    sphere.move_global(15.0, 0.0, 15.0);
    s.add_object(sphere);

    let start = std::time::Instant::now();
    c.render(&s, 1);

    let dur = start.elapsed().as_secs_f64();
    println!("rendered in {:.2} sec", dur);
}
