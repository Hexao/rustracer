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

fn main() {
    let scn = match std::env::args().nth(1) {
        Some(scn) => scn,
        None => {
            println!("expected scene, found nothing!");
            return;
        }
    };

    let file = match std::fs::File::open(scn) {
        Ok(file) => file,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let reader = std::io::BufReader::new(file);
    let json: Map<String, Value> = match serde_json::from_reader(reader) {
        Ok(map) => map,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let mut scene = Scene::new();
    let mut camera = Camera::new(1920, 1080, Focal::Perspective(1.7));

    if let Some(objects) = json.get("objects") {
        let objects = match objects.as_array() {
            Some(objects) => objects,
            None => {
                println!("The key \"objects\" should be an array!");
                return;
            }
        };

        for (id, object) in objects.iter().enumerate() {
            let object = object.as_object().unwrap_or_else(|| {
                panic!("Object no {} in \"objects\" should be objects structure!", id);
            });

            let obj_type = object.get("type").unwrap_or_else(|| {
                panic!("Object no {} should have a \"type\"", id);
            }).as_str().unwrap_or_else(|| {
                panic!("Object no {}, the type must be a string!", id);
            });

            let material = object.get("material").unwrap_or_else(|| {
                panic!("Object no {} should have a \"material\"", id);
            }).as_object().unwrap_or_else(|| {
                panic!("Object no {}, the material must be an object!", id);
            });

            let mat_type = material.get("type").unwrap_or_else(|| {
                panic!("Material of object no {} should have a \"type\"", id);
            }).as_str().unwrap_or_else(|| {
                panic!("Object no {}, the material type must be an object!", id);
            });

            let mat_object: Box<dyn MatProvider> = match mat_type {
                "SIMPLE" => {
                    let material = material.get("mat").unwrap_or_else(|| {
                        panic!("Object no {}, simple_mat should have a \"mat\"", id);
                    }).as_object().unwrap_or_else(|| {
                        panic!("Object no {}, mat must be an object", id);
                    });

                    let ambient = get_color(material, "ambient").unwrap_or_else(|e| {
                        panic!("Object no {}, {}", id, e);
                    });
                    let diffuse = get_color(material, "ambient").unwrap_or_else(|e| {
                        panic!("Object no {}, {}", id, e);
                    });
                    let specular = get_color(material, "ambient").unwrap_or_else(|e| {
                        panic!("Object no {}, {}", id, e);
                    });
                    let alpha = material.get("alpha").unwrap_or_else(|| {
                        panic!("Object no {}, should have a \"alpha\"", id);
                    }).as_u64().unwrap_or_else(|| {
                        panic!("Object on {}, alpha must be an integer", id);
                    }) as u8;
                    let shininess = material.get("shininess").unwrap_or_else(|| {
                        panic!("Object no {}, should have a \"shininess\"", id);
                    }).as_f64().unwrap_or_else(|| {
                        panic!("Object on {}, alpha must be an integer", id);
                    }) as f32;

                    Box::new(SimpleMat::new(Material::new(ambient, diffuse, specular, alpha, shininess)))
                }
                "TEXTURE" => {
                    let file_name = material.get("resource").unwrap_or_else(|| {
                        panic!("Object no {}, texture should have a \"resource\"", id);
                    }).as_str().unwrap_or_else(|| {
                        panic!("Object no {}, resource must be a string", id);
                    });
                    let shininess = material.get("shininess").unwrap_or_else(|| {
                        panic!("Object no {}, should have a \"shininess\"", id);
                    }).as_f64().unwrap_or_else(|| {
                        panic!("Object on {}, alpha must be an integer", id);
                    }) as f32;

                    Box::new(Texture::new(file_name, shininess))
                }
                unknown => panic!("Object no {}, fonud an unknown material type: {}", id, unknown),
            };

            let mut scn_object: Box<dyn Object> = match obj_type {
                "SPHERE" => Box::new(Sphere::new(mat_object)),
                "PLANE" => Box::new(Plane::new(mat_object)),
                unknown => panic!("Object no {} has an unknown type: \"{}\"", id, unknown),
            };

            if let Some(transform) = object.get("transform") {
                let transform = transform.as_array().unwrap_or_else(|| {
                    panic!("Object no {}, transform must be an array of size 3", id);
                });

                if transform.len() == 3 {
                    let x = transform[0].as_f64().unwrap_or_else(|| {
                        panic!("Object no {}, transform contains a none floting point value", id)
                    }) as f32;
                    let y = transform[1].as_f64().unwrap_or_else(|| {
                        panic!("Object no {}, transform contains a none floting point value", id)
                    }) as f32;
                    let z = transform[2].as_f64().unwrap_or_else(|| {
                        panic!("Object no {}, transform contains a none floting point value", id)
                    }) as f32;

                    scn_object.move_global(x, y, z);
                } else {
                    panic!("Object no {}, transform must be an array of size 3", id);
                }
            }

            if let Some(scale) = object.get("scale") {
                let scale = scale.as_f64().unwrap_or_else(|| {
                    panic!("Object no {}, scale must be a floting point value", id);
                }) as f32;

                scn_object.scale(scale);
            }

            scene.add_object(scn_object);
        }
    }

    camera.render(&scene, 6);
}

fn get_color(map: &Map<String, Value>, key: &str) -> Result<Color, String> {
    let value = match map.get(key) {
        Some(value) => value,
        None => return Err(format!("should have a \"{}\"", key)),
    };
    
    match value.as_u64() {
        Some(gray) => Ok(Color::new_gray(gray as u8)),
        None => match value.as_array() {
            Some(array) => {
                if array.len() == 3 {
                    let red = match array[0].as_u64() {
                        Some(red) => red as u8,
                        None => return Err(format!("{} contain an none integer value!", key))
                    };
                    let green = match array[1].as_u64() {
                        Some(red) => red as u8,
                        None => return Err(format!("{} contain an none integer value!", key))
                    };
                    let blue = match array[2].as_u64() {
                        Some(red) => red as u8,
                        None => return Err(format!("{} contain an none integer value!", key))
                    };

                    Ok(Color::new(red, green, blue))
                } else {
                    Err(format!("array for {} must be of size 3!", key))
                }
            }
            None => Err(format!("{} must be an integer or array of size 3!", key))
        }
    }
}
