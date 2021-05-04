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

use serde_json::{Map, Value};
use std::fmt::Display;

fn main() {
    let path = match std::env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("usage: rustracer.exe <scene.json>");
            return;
        }
    };

    let file = match std::fs::File::open(path.clone()) {
        Ok(file) => file,
        Err(e) => {
            println!("{}: {}", path, e);
            return;
        }
    };

    let reader = std::io::BufReader::new(file);
    let json: Map<String, Value> = match serde_json::from_reader(reader) {
        Ok(json) => json,
        Err(e) => {
            println!("in json file: {}", e);
            return;
        }
    };

    let mut scene = Scene::new();

    if let Err(e) = parse_objects(&mut scene, &json) {
        println!("{}", e);
        return;
    }
    if let Err(e) = parse_lights(&mut scene, &json) {
        println!("{}", e);
        return;
    }
    let camera = match parse_camera(&json) {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(camera) => camera,
    };
    let conf = match parse_config(&json) {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(conf) => conf,
    };

    camera.render_in(&scene, conf.output.as_str(), conf.threads);
}

enum Malformed {
    NotAnArray(&'static str),
    NotAnObject(&'static str),
    Object(usize, InnerError),
    Light(usize, InnerError),
    Camera(InnerError),
    Config(InnerError),
}

impl Display for Malformed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Malformed::NotAnArray(category) => write!(f, "\"{}\" should be an array", category)?,
            Malformed::NotAnObject(category) => write!(f, "\"{}\" should be an object", category)?,
            Malformed::Object(id, err) => write!(f, "Object {}{}", id, err)?,
            Malformed::Light(id, err) => write!(f, "Light {}{}", id, err)?,
            Malformed::Camera(err) => write!(f, "Camera {}", err)?,
            Malformed::Config(err) => write!(f, "Config {}", err)?,
        }

        Ok(())
    }
}

enum InnerError {
    NotAnObject,
    MissingField(&'static str),
    FieldArray(&'static str, &'static str, usize),
    FieldFloat(&'static str),
    FieldInteger(&'static str),
    FieldString(&'static str),
    FieldObject(&'static str),
    FieldColor(&'static str),
    UnknownMaterial(String),
    UnknownFocal(String),
    UnknownType(String),
    UnknownFlag(String),
}

impl Display for InnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InnerError::NotAnObject => write!(f, " should be an Object")?,
            InnerError::MissingField(field) => write!(f, " should have a \"{}\"", field)?,
            InnerError::FieldArray(field, array_type, size) => write!(f, ", \"{}\" must be a {}Array of size {}", field, array_type, size)?,
            InnerError::FieldFloat(field) => write!(f, ", \"{}\" must be a Float", field)?,
            InnerError::FieldInteger(field) => write!(f, ", \"{}\" must be an Integer", field)?,
            InnerError::FieldString(field) => write!(f, ", \"{}\" must be a String", field)?,
            InnerError::FieldObject(field) => write!(f, ", \"{}\" must be an Object", field)?,
            InnerError::FieldColor(field) => write!(f, ", \"{}\" must be an Int or an IntArray of size 3!", field)?,
            InnerError::UnknownMaterial(material) => write!(f, ", fonud an unknown material type: {}", material)?,
            InnerError::UnknownFocal(focal) => write!(f, " has an unknown focal type: {}", focal)?,
            InnerError::UnknownType(obj_type) => write!(f, " has an unknown type: {}", obj_type)?,
            InnerError::UnknownFlag(flag) => write!(f, " has an unknown flag: {}", flag)?,
        }

        Ok(())
    }
}

fn parse_objects(scene: &mut Scene, json: &Map<String, Value>) -> Result<(), Malformed> {
    if let Some(objects) = json.get("objects") {
        let objects = match objects.as_array() {
            None => return Err(Malformed::NotAnArray("object")),
            Some(objects) => objects,
        };

        for (id, object) in objects.iter().enumerate() {
            let object = match object.as_object() {
                None => return Err(Malformed::Object(id, InnerError::NotAnObject)),
                Some(object) => object,
            };

            let obj_type = match match object.get("type") {
                None => return Err(Malformed::Object(id, InnerError::MissingField("type"))),
                Some(obj) => obj,
            }.as_str() {
                None => return Err(Malformed::Object(id, InnerError::FieldString("type"))),
                Some(obj_type) => obj_type,
            };

            let material = match match object.get("material") {
                None => return Err(Malformed::Object(id, InnerError::MissingField("material"))),
                Some(material) => material,
            }.as_object() {
                None => return Err(Malformed::Object(id, InnerError::FieldObject("material"))),
                Some(material) => material,
            };

            let mat_type = match match material.get("type") {
                None => return Err(Malformed::Object(id, InnerError::MissingField("material/type"))),
                Some(mat_type) => mat_type,
            }.as_str() {
                None => return Err(Malformed::Object(id, InnerError::FieldString("material/type"))),
                Some(mat_type) => mat_type,
            };

            let mat_object: Box<dyn MatProvider> = match mat_type {
                "SIMPLE" => {
                    let material = match match material.get("mat") {
                        None => return Err(Malformed::Object(id, InnerError::MissingField("material/mat"))),
                        Some(material) => material,
                    }.as_object() {
                        None => return Err(Malformed::Object(id, InnerError::FieldObject("material/mat"))),
                        Some(material) => material,
                    };

                    let ambient = match get_color(material, "ambient", "material/mat/ambient") {
                        Err(obj_err) => return Err(Malformed::Object(id, obj_err)),
                        Ok(color) => color,
                    };
                    let diffuse = match get_color(material, "diffuse", "material/mat/diffuse") {
                        Err(obj_err) => return Err(Malformed::Object(id, obj_err)),
                        Ok(color) => color,
                    };
                    let specular = match get_color(material, "specular", "material/mat/specular") {
                        Err(obj_err) => return Err(Malformed::Object(id, obj_err)),
                        Ok(color) => color,
                    };
                    let alpha = match material.get("alpha") {
                        Some(alpha) => match alpha.as_u64() {
                            None => return Err(Malformed::Object(id, InnerError::FieldInteger("material/mat/alpha"))),
                            Some(alpha) => alpha as u8,
                        },
                        None => 255,
                    };
                    let shininess = match material.get("shininess") {
                        Some(shininess) => match shininess.as_f64() {
                            None => return Err(Malformed::Object(id, InnerError::FieldInteger("material/mat/shininess"))),
                            Some(shininess) => shininess as f32,
                        },
                        None => 50.0,
                    };

                    Box::new(SimpleMat::new(Material::new(ambient, diffuse, specular, alpha, shininess)))
                }
                "TEXTURE" => {
                    let file_name = match match material.get("resource") {
                        None => return Err(Malformed::Object(id, InnerError::MissingField("material/resource"))),
                        Some(file_name) => file_name,
                    }.as_str() {
                        None => return Err(Malformed::Object(id, InnerError::FieldString("material/resource"))),
                        Some(file_name) => file_name,
                    };

                    let shininess = match material.get("shininess") {
                        Some(shininess) => match shininess.as_f64() {
                            None => return Err(Malformed::Object(id, InnerError::FieldInteger("material/shininess"))),
                            Some(shininess) => shininess as f32,
                        },
                        None => 50.0,
                    };

                    Box::new(Texture::new(file_name, shininess))
                }
                unknown => return Err(Malformed::Object(id, InnerError::UnknownMaterial(unknown.to_owned()))),
            };

            let mut scn_object: Box<dyn Object> = match obj_type {
                "SPHERE" => Box::new(Sphere::new(mat_object)),
                "PLANE" => Box::new(Plane::new(mat_object)),
                unknown => return Err(Malformed::Object(id, InnerError::UnknownType(unknown.to_owned()))),
            };

            if let Some(transform) = object.get("transform") {
                let transform = match transform.as_array() {
                    None => return Err(Malformed::Object(id, InnerError::FieldArray("transform", "Float", 3))),
                    Some(transform) => transform,
                };

                if transform.len() == 3 {
                    let x = match transform[0].as_f64() {
                        None => return Err(Malformed::Object(id, InnerError::FieldArray("transform", "Float", 3))),
                        Some(x) => x as f32,
                    };
                    let y = match transform[1].as_f64() {
                        None => return Err(Malformed::Object(id, InnerError::FieldArray("transform", "Float", 3))),
                        Some(y) => y as f32,
                    };
                    let z = match transform[2].as_f64() {
                        None => return Err(Malformed::Object(id, InnerError::FieldArray("transform", "Float", 3))),
                        Some(z) => z as f32,
                    };

                    scn_object.move_global(x, y, z);
                } else {
                    return Err(Malformed::Object(id, InnerError::FieldArray("transform", "Float", 3)));
                }
            }

            if let Some(scale) = object.get("scale") {
                let scale = match scale.as_f64() {
                    None => return Err(Malformed::Object(id, InnerError::FieldFloat("scale"))),
                    Some(scale) => scale as f32,
                };

                scn_object.scale(scale);
            }

            scene.add_object(scn_object);
        }
    }

    Ok(())
}

fn parse_lights(scene: &mut Scene, json: &Map<String, Value>) -> Result<(), Malformed> {
    if let Some(lights) = json.get("lights") {
        let lights = match lights.as_array() {
            Some(lights) => lights,
            None => return Err(Malformed::NotAnArray("lights")),
        };

        for (id, light) in lights.iter().enumerate() {
            let light = match light.as_object() {
                Some(light) => light,
                None => return Err(Malformed::Light(id, InnerError::NotAnObject)),
            };

            let lig_type = match match light.get("type") {
                Some(lig) => lig,
                None => return Err(Malformed::Light(id, InnerError::MissingField("type"))),
            }.as_str() {
                Some(lig_type) => lig_type,
                None => return Err(Malformed::Light(id, InnerError::FieldString("type"))),
            };

            let color = match match light.get("color") {
                None => return Err(Malformed::Light(id, InnerError::MissingField("color"))),
                Some(color) => color,
            }.as_object() {
                None => return Err(Malformed::Light(id, InnerError::FieldObject("color"))),
                Some(color) => color,
            };

            let diffuse = match get_color(color, "diffuse", "color/diffuse") {
                Err(inner) => return Err(Malformed::Light(id, inner)),
                Ok(diffuse) => diffuse,
            };
            let specular = match get_color(color, "specular", "color/specular") {
                Err(inner) => return Err(Malformed::Light(id, inner)),
                Ok(specular) => specular,
            };

            let mut scn_light: Box<dyn Light> = match lig_type {
                "POINT" => Box::new(PointLight::new(diffuse, specular)),
                unknown => return Err(Malformed::Light(id, InnerError::UnknownType(unknown.to_owned()))),
            };

            if let Some(transform) = light.get("transform") {
                let transform = match transform.as_array() {
                    None => return Err(Malformed::Light(id, InnerError::FieldArray("transform", "Float", 3))),
                    Some(transform) => transform,
                };

                if transform.len() == 3 {
                    let x = match transform[0].as_f64() {
                        None => return Err(Malformed::Light(id, InnerError::FieldArray("transform", "Float", 3))),
                        Some(x) => x as f32,
                    };
                    let y = match transform[1].as_f64() {
                        None => return Err(Malformed::Light(id, InnerError::FieldArray("transform", "Float", 3))),
                        Some(y) => y as f32,
                    };
                    let z = match transform[2].as_f64() {
                        None => return Err(Malformed::Light(id, InnerError::FieldArray("transform", "Float", 3))),
                        Some(z) => z as f32,
                    };

                    scn_light.move_global(x, y, z);
                } else {
                    return Err(Malformed::Light(id, InnerError::FieldArray("transform", "Float", 3)));
                }
            }

            scene.add_light(scn_light);
        }
    }

    Ok(())
}

fn parse_camera(json: &Map<String, Value>) -> Result<Camera, Malformed> {
    if let Some(camera) = json.get("camera") {
        let camera = match camera.as_object() {
            Some(camera) => camera,
            None => return Err(Malformed::NotAnObject("camera")),
        };

        let [x, y] = match camera.get("size") {
            None => [1920, 1080],
            Some(size) => match size.as_array() {
                None => return Err(Malformed::Camera(InnerError::FieldArray("size", "Unsigned", 2))),
                Some(size) => {
                    if size.len() == 2 {
                        let x = match size[0].as_u64() {
                            None => return Err(Malformed::Camera(InnerError::FieldArray("size", "Unsigned", 2))),
                            Some(x) => x as usize,
                        };
                        let y = match size[1].as_u64() {
                            None => return Err(Malformed::Camera(InnerError::FieldArray("size", "Unsigned", 2))),
                            Some(y) => y as usize,
                        };
                        [x, y]
                    } else {
                        return Err(Malformed::Camera(InnerError::FieldArray("size", "Unsigned", 2)));
                    }
                }
            }
        };

        let focal = match camera.get("focal") {
            None => Focal::Perspective(1.7),
            Some(focal) => match focal.as_object() {
                None => return Err(Malformed::Camera(InnerError::FieldObject("focal"))),
                Some(focal) => {
                    let focal_type = match match focal.get("type") {
                        None => return Err(Malformed::Camera(InnerError::MissingField("focal/type"))),
                        Some(focal) => focal,
                    }.as_str() {
                        None => return Err(Malformed::Camera(InnerError::FieldString("focal/type"))),
                        Some(focal) => focal,
                    };

                    let size = match match focal.get("size") {
                        None => return Err(Malformed::Camera(InnerError::MissingField("focal/size"))),
                        Some(size) => size,
                    }.as_f64() {
                        None => return Err(Malformed::Camera(InnerError::FieldFloat("focal/size"))),
                        Some(size) => size as f32,
                    };

                    match focal_type {
                        "PERSPECTIVE" => Focal::Perspective(size),
                        "ORTHOGRAPHIC" => Focal::Orthographic(size),
                        unknown => return Err(Malformed::Camera(InnerError::UnknownFocal(unknown.to_owned()))),
                    }
                }
            }
        };

        let flags = match camera.get("flags") {
            None => 0,
            Some(flags) => match flags.as_array() {
                None => return Err(Malformed::Camera(InnerError::FieldArray("flags", "String", 0))),
                Some(flags_vec) => {
                    let mut flags = 0;

                    for flag in flags_vec {
                        let flag = match flag.as_str() {
                            None => return Err(Malformed::Camera(InnerError::FieldArray("flags", "String", 0))),
                            Some(flag) => flag,
                        };

                        match flag {
                            "ANTI_ALIASING" => flags |= Camera::ANTI_ALIASING,
                            unknown => return Err(Malformed::Camera(InnerError::UnknownFlag(unknown.to_owned()))),
                        }
                    }

                    flags
                }
            }
        };

        let mut camera = Camera::new(x, y, focal);
        camera.set_flags(flags);

        Ok(camera)
    } else {
        Ok(Camera::new(1920, 1080, Focal::Perspective(1.7)))
    }
}

struct Config {
    output: String,
    threads: usize,
}

fn parse_config(json: &Map<String, Value>) -> Result<Config, Malformed> {
    if let Some(conf) = json.get("config") {
        let conf = match conf.as_object() {
            None => return Err(Malformed::NotAnObject("config")),
            Some(conf) => conf,
        };

        let output = match conf.get("output") {
            None => "output.png".to_owned(),
            Some(output) => match output.as_str() {
                None => return Err(Malformed::Config(InnerError::FieldString("output"))),
                Some(output) => output.to_owned(),
            }
        };
        let threads = match conf.get("threads") {
            None => 1,
            Some(threads) => match threads.as_u64() {
                None => return Err(Malformed::Config(InnerError::FieldInteger("threads"))),
                Some(threads) => threads as usize,
            }
        };

        Ok(Config { output, threads })
    } else {
        Ok(Config { output: "output.png".to_owned(), threads: 1 })
    }
}

fn get_color(map: &Map<String, Value>, key: &'static str, field: &'static str) -> Result<Color, InnerError> {
    let value = match map.get(key) {
        Some(value) => value,
        None => return Err(InnerError::MissingField(field)),
    };
    
    match value.as_u64() {
        Some(gray) => Ok(Color::new_gray(gray as u8)),
        None => match value.as_array() {
            Some(array) => {
                if array.len() == 3 {
                    let red = match array[0].as_u64() {
                        Some(red) => red as u8,
                        None => return Err(InnerError::FieldColor(field)),
                    };
                    let green = match array[1].as_u64() {
                        Some(red) => red as u8,
                        None => return Err(InnerError::FieldColor(field)),
                    };
                    let blue = match array[2].as_u64() {
                        Some(red) => red as u8,
                        None => return Err(InnerError::FieldColor(field)),
                    };

                    Ok(Color::new(red, green, blue))
                } else {
                    Err(InnerError::FieldColor(field))
                }
            }
            None => Err(InnerError::FieldColor(field))
        }
    }
}
