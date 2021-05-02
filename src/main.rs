pub mod material;
pub mod object;
pub mod scene;
pub mod math;

use object::{
    light::{Light, point_light::PointLight},
    camera::{Camera, Focal},
    sphere::Sphere,
    plane::Plane,
    Object,
};

use material::{
    simple_mat::SimpleMat,
    texture::Texture,
    MatProvider,
    Material,
    Color,
};

use scene::Scene;

fn main() {
    let mut c = Camera::new(1920, 1080, Focal::Perspective(1.7));
    // let c = Camera::new(7680, 4320, Focal::Perspective(1.7));
    // let c = Camera::new(720, 480, Focal::Perspective(1.7));
    let mut s = Scene::new();

    c.set_flags(0);

    let texture = Texture::new("pito.png", 70.0);

    let white_mat = Material::new(
        Color::GRAY,
        Color::WHITE,
        Color::FULL_WHITE,
        255,
        75.0
    );

    let red_mat = Material::new(
        Color::DARK_RED,
        Color::RED,
        Color::new(255, 120, 120),
        255,
        50.0
    );

    let green_mat = Material::new(
        Color::DARK_GREEN,
        Color::GREEN,
        Color::new(120, 255, 120),
        255,
        50.0
    );

    let blue_mat = Material::new(
        Color::DARK_BLUE,
        Color::BLUE,
        Color::new(120, 120, 255),
        255,
        50.0
    );

    let color: Box<dyn MatProvider> = Box::new(SimpleMat::new(white_mat));
    let mut sphere: Box<dyn Object> = Box::new(Sphere::new(color));
    sphere.move_global(0.0, 0.0, 15.0);
    s.add_object(sphere);

    // let color: Box<dyn MatProvider> = Box::new(SimpleMat::new(red_mat));
    let color: Box<dyn MatProvider> = Box::new(texture);
    let mut sphere: Box<dyn Object> = Box::new(Plane::new(color));
    sphere.move_global(2.5, 0.0, 15.0);
    sphere.scale(2.25);
    // sphere.rotate_y(120.0);
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

    let mut light: Box<dyn Light> = Box::new(PointLight::new(Color::WHITE, Color::FULL_WHITE));
    light.move_global(2.5, 7.5, 0.0);
    s.add_light(light);

    let start = std::time::Instant::now();
    c.render(&s, 6);

    let dur = start.elapsed().as_secs_f64();
    println!("rendered in {:.2} sec", dur);
}
