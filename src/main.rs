mod material;
mod parser;
mod object;
mod scene;
mod math;

fn main() {
    let path = match std::env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("usage: rustracer.exe <scene.json>");
            return;
        }
    };

    let (scene, camera, config) = parser::parse_file(path.as_str());
    camera.render_in(&scene, config.output.as_str(), config.depth, config.threads);
}
