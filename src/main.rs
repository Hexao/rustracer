pub mod material;
pub mod object;
pub mod scene;
pub mod math;

use object::{
    light::{Light, point_light::PointLight},
    camera::{Camera, Focal},
    sphere::Sphere,
    Object,
};

use material::{
    simple_mat::SimpleMat,
    MatProvider,
    Material,
    Color,
};

use scene::Scene;

fn main() {
    let c = Camera::new(1920, 1080, Focal::Perspective(1.75));
    let mut s = Scene::new();

    let white_mat = Material::new(
        Color::GRAY,
        Color::WHITE,
        Color::FULL_WHITE,
        255,
        10.0
    );

    let red_mat = Material::new(
        Color::DARK_RED,
        Color::RED,
        Color::FULL_RED,
        255,
        10.0
    );

    let green_mat = Material::new(
        Color::DARK_GREEN,
        Color::GREEN,
        Color::FULL_GREEN,
        255,
        10.0
    );

    let blue_mat = Material::new(
        Color::DARK_BLUE,
        Color::BLUE,
        Color::FULL_BLUE,
        255,
        10.0
    );

    let color: Box<dyn MatProvider> = Box::new(SimpleMat::new(white_mat));
    let mut sphere: Box<dyn Object> = Box::new(Sphere::new(color));
    sphere.move_global(0.0, 0.0, 15.0);
    s.add_object(sphere);

    let color: Box<dyn MatProvider> = Box::new(SimpleMat::new(red_mat));
    let mut sphere: Box<dyn Object> = Box::new(Sphere::new(color));
    sphere.move_global(2.5, 0.0, 15.0);
    s.add_object(sphere);

    let color: Box<dyn MatProvider> = Box::new(SimpleMat::new(green_mat));
    let mut sphere: Box<dyn Object> = Box::new(Sphere::new(color));
    sphere.move_global(5.0, 0.0, 15.0);
    s.add_object(sphere);

    let color: Box<dyn MatProvider> = Box::new(SimpleMat::new(blue_mat));
    let mut sphere: Box<dyn Object> = Box::new(Sphere::new(color));
    sphere.move_global(7.5, 0.0, 15.0);
    s.add_object(sphere);

    let color: Box<dyn MatProvider> = Box::new(SimpleMat::new(blue_mat));
    let mut sphere: Box<dyn Object> = Box::new(Sphere::new(color));
    sphere.move_global(-2.5, 0.0, 15.0);
    s.add_object(sphere);

    let color: Box<dyn MatProvider> = Box::new(SimpleMat::new(green_mat));
    let mut sphere: Box<dyn Object> = Box::new(Sphere::new(color));
    sphere.move_global(-5.0, 0.0, 15.0);
    s.add_object(sphere);

    let color: Box<dyn MatProvider> = Box::new(SimpleMat::new(red_mat));
    let mut sphere: Box<dyn Object> = Box::new(Sphere::new(color));
    sphere.move_global(-7.5, 0.0, 15.0);
    s.add_object(sphere);

    let light: Box<dyn Light> = Box::new(PointLight::new(Color::WHITE, Color::FULL_WHITE));
    s.add_light(light);

    let start = std::time::Instant::now();
    c.render(&s, 6);

    let dur = start.elapsed().as_secs_f64();
    println!("rendered in {:.2} sec", dur);
}
