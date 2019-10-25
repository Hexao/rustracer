#![allow(dead_code, unused_imports, unused_variables)]

mod object;
mod camera;
mod sphere;
mod scene;
mod ray;

use object::Object;
use camera::Camera;
use scene::Scene;

fn main() {
    let c = Camera::new(1920, 1080, 1.0);
    let s = Scene::new();

    c.render(&s);
}
