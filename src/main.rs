pub mod scene;

pub mod math;
pub mod object;
pub mod material;

use object::{
    Object,
    sphere::Sphere,
    camera::{Camera, Focal},
};

use material::{
    Color,
    Material,
    simple_mat::SimpleMat,
};

use scene::Scene;

fn main() {
    let c = Camera::new(1920, 1080, Focal::Perspective(1.75));
    let mut s = Scene::new();

    let color: Box<dyn Material> = Box::new(SimpleMat::new(Color::new(255, 255, 255)));
    let mut sphere: Box<dyn Object> = Box::new(Sphere::new(color));
    sphere.move_global(0.0, 0.0, 15.0);
    s.add_object(sphere);

    let color: Box<dyn Material> = Box::new(SimpleMat::new(Color::new(255, 0, 0)));
    let mut sphere: Box<dyn Object> = Box::new(Sphere::new(color));
    sphere.move_global(2.5, 0.0, 15.0);
    s.add_object(sphere);

    let color: Box<dyn Material> = Box::new(SimpleMat::new(Color::new(0, 255, 0)));
    let mut sphere: Box<dyn Object> = Box::new(Sphere::new(color));
    sphere.move_global(5.0, 0.0, 15.0);
    s.add_object(sphere);

    let color: Box<dyn Material> = Box::new(SimpleMat::new(Color::new(0, 0, 255)));
    let mut sphere: Box<dyn Object> = Box::new(Sphere::new(color));
    sphere.move_global(7.5, 0.0, 15.0);
    s.add_object(sphere);

    let color: Box<dyn Material> = Box::new(SimpleMat::new(Color::new(0, 255, 255)));
    let mut sphere: Box<dyn Object> = Box::new(Sphere::new(color));
    sphere.move_global(-2.5, 0.0, 15.0);
    s.add_object(sphere);

    let color: Box<dyn Material> = Box::new(SimpleMat::new(Color::new(255, 0, 255)));
    let mut sphere: Box<dyn Object> = Box::new(Sphere::new(color));
    sphere.move_global(-5.0, 0.0, 15.0);
    s.add_object(sphere);

    let color: Box<dyn Material> = Box::new(SimpleMat::new(Color::new(255, 255, 0)));
    let mut sphere: Box<dyn Object> = Box::new(Sphere::new(color));
    sphere.move_global(-7.5, 0.0, 15.0);
    s.add_object(sphere);

    let start = std::time::Instant::now();
    c.render(&s, 6);

    let dur = start.elapsed().as_secs_f64();
    println!("rendered in {:.2} sec", dur);
}
