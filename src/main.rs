mod material;
mod object;
mod scene;
mod math;

use object::{
    light::{Light, point_light::PointLight},
    camera::{Camera, Focal},
    sphere::Sphere,
    plane::Plane,
    Movable,
    Object,
};

use material::{
    strip_x_mat::StripXMat,
    strip_y_mat::StripYMat,
    simple_mat::SimpleMat,
    grid_mat::GridMat,
    texture::Texture,
    MatProvider,
    Material,
    Color,
};

use scene::Scene;

use serde_json::{Map, Value};
use std::fmt::Display;

fn main() {
    // TODO: fix hcoord norm :grimacing:

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

    let mut scene = match parse_scene(&json) {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(scene) => scene,
    };

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

    camera.render_in(&scene, conf.output.as_str(), conf.depth, conf.threads);
}

enum Malformed {
    NotAnArray(&'static str),
    NotAnObject(&'static str),
    Object(usize, InnerError),
    Light(usize, InnerError),
    Camera(InnerError),
    Config(InnerError),
    Scene(InnerError),
}

impl Display for Malformed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Malformed::NotAnArray(category) => write!(f, "\"{}\" should be an array", category)?,
            Malformed::NotAnObject(category) => write!(f, "\"{}\" should be an object", category)?,
            Malformed::Object(id, err) => write!(f, "Object {}{}", id, err)?,
            Malformed::Light(id, err) => write!(f, "Light {}{}", id, err)?,
            Malformed::Camera(err) => write!(f, "Camera{}", err)?,
            Malformed::Config(err) => write!(f, "Config{}", err)?,
            Malformed::Scene(err) => write!(f, "Scene{}", err)?,
        }

        Ok(())
    }
}

enum InnerError {
    NotAnObject,
    MissingField(&'static str),
    FieldArray(&'static str, &'static str, usize),
    WrongField(&'static str, &'static str),
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
            InnerError::FieldArray(field, array_type, size) if *size > 0 => write!(f, ", \"{}\" must be a {}Array of size {}", field, array_type, size)?,
            InnerError::FieldArray(field, array_type, _) => write!(f, ", \"{}\" must be a {}Array", field, array_type)?,
            InnerError::WrongField(field, expected_type) => write!(f, ", \"{}\" must be a {}", field, expected_type)?,
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
                None => return Err(Malformed::Object(id, InnerError::WrongField("type", "String"))),
                Some(obj_type) => obj_type,
            };

            let material = match match object.get("material") {
                None => return Err(Malformed::Object(id, InnerError::MissingField("material"))),
                Some(material) => material,
            }.as_object() {
                None => return Err(Malformed::Object(id, InnerError::WrongField("material", "Object"))),
                Some(material) => material,
            };

            let mat_type = match match material.get("type") {
                None => return Err(Malformed::Object(id, InnerError::MissingField("material/type"))),
                Some(mat_type) => mat_type,
            }.as_str() {
                None => return Err(Malformed::Object(id, InnerError::WrongField("material/type", "String"))),
                Some(mat_type) => mat_type,
            };

            let mat_object: Box<dyn MatProvider> = match mat_type {
                "SIMPLE" => {
                    let material = match match material.get("mat") {
                        None => return Err(Malformed::Object(id, InnerError::MissingField("material/mat"))),
                        Some(material) => material,
                    }.as_object() {
                        None => return Err(Malformed::Object(id, InnerError::WrongField("material/mat", "Object"))),
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
                        None => 255,
                        Some(alpha) => match alpha.as_u64() {
                            None => return Err(Malformed::Object(id, InnerError::WrongField("material/mat/alpha", "u8"))),
                            Some(alpha) => alpha as u8,
                        },
                    };
                    let reflection = match material.get("reflection") {
                        None => 0,
                        Some(reflection) => match reflection.as_u64() {
                            None => return Err(Malformed::Object(id, InnerError::WrongField("material/mat/reflection", "u8"))),
                            Some(shininess) => shininess as u8,
                        },
                    };
                    let shininess = match material.get("shininess") {
                        None => 50.0,
                        Some(shininess) => match shininess.as_f64() {
                            None => return Err(Malformed::Object(id, InnerError::WrongField("material/mat/shininess", "Float"))),
                            Some(shininess) => shininess as f32,
                        },
                    };

                    Box::new(SimpleMat::new(Material::new(ambient, diffuse, specular, alpha, reflection, shininess)))
                }
                "STRIP_X" => {
                    let materials = match match material.get("mat") {
                        None => return Err(Malformed::Object(id, InnerError::MissingField("material/mat"))),
                        Some(material) => material,
                    }.as_array() {
                        None => return Err(Malformed::Object(id, InnerError::FieldArray("material/mat", "Material", 2))),
                        Some(material) => material,
                    };

                    let mut mat: [Material; 2] = [
                        Material::default(), Material::default()
                    ];

                    for (mat_id, material) in materials.iter().enumerate() {
                        let material = match material.as_object() {
                            None => return Err(Malformed::Object(id, InnerError::WrongField("material/mat/<element>", "Object"))),
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
                            None => 255,
                            Some(alpha) => match alpha.as_u64() {
                                None => return Err(Malformed::Object(id, InnerError::WrongField("material/mat/alpha", "u8"))),
                                Some(alpha) => alpha as u8,
                            },
                        };
                        let reflection = match material.get("reflection") {
                            None => 0,
                            Some(reflection) => match reflection.as_u64() {
                                None => return Err(Malformed::Object(id, InnerError::WrongField("material/mat/reflection", "u8"))),
                                Some(shininess) => shininess as u8,
                            },
                        };
                        let shininess = match material.get("shininess") {
                            None => 50.0,
                            Some(shininess) => match shininess.as_f64() {
                                None => return Err(Malformed::Object(id, InnerError::WrongField("material/mat/shininess", "Float"))),
                                Some(shininess) => shininess as f32,
                            },
                        };

                        mat[mat_id] = Material::new(ambient, diffuse, specular, alpha, reflection, shininess);
                    }
                    
                    let rep = match material.get("rep") {
                        None => 1,
                        Some(alpha) => match alpha.as_u64() {
                            None => return Err(Malformed::Object(id, InnerError::WrongField("material/rep", "Unsigned"))),
                            Some(rep) => rep as usize,
                        },
                    };

                    Box::new(StripXMat::new(mat[0], mat[1], rep))
                }
                "STRIP_Y" => {
                    let materials = match match material.get("mat") {
                        None => return Err(Malformed::Object(id, InnerError::MissingField("material/mat"))),
                        Some(material) => material,
                    }.as_array() {
                        None => return Err(Malformed::Object(id, InnerError::FieldArray("material/mat", "Material", 2))),
                        Some(material) => material,
                    };

                    let mut mat: [Material; 2] = [
                        Material::default(), Material::default()
                    ];

                    for (mat_id, material) in materials.iter().enumerate() {
                        let material = match material.as_object() {
                            None => return Err(Malformed::Object(id, InnerError::WrongField("material/mat/<element>", "Object"))),
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
                            None => 255,
                            Some(alpha) => match alpha.as_u64() {
                                None => return Err(Malformed::Object(id, InnerError::WrongField("material/mat/alpha", "u8"))),
                                Some(alpha) => alpha as u8,
                            },
                        };
                        let reflection = match material.get("reflection") {
                            None => 0,
                            Some(reflection) => match reflection.as_u64() {
                                None => return Err(Malformed::Object(id, InnerError::WrongField("material/mat/reflection", "u8"))),
                                Some(shininess) => shininess as u8,
                            },
                        };
                        let shininess = match material.get("shininess") {
                            None => 50.0,
                            Some(shininess) => match shininess.as_f64() {
                                None => return Err(Malformed::Object(id, InnerError::WrongField("material/mat/shininess", "Float"))),
                                Some(shininess) => shininess as f32,
                            },
                        };

                        mat[mat_id] = Material::new(ambient, diffuse, specular, alpha, reflection, shininess);
                    }
                    
                    let rep = match material.get("rep") {
                        None => 1,
                        Some(alpha) => match alpha.as_u64() {
                            None => return Err(Malformed::Object(id, InnerError::WrongField("material/rep", "Unsigned"))),
                            Some(rep) => rep as usize,
                        },
                    };

                    Box::new(StripYMat::new(mat[0], mat[1], rep))
                }
                "GRID" => {
                    let materials = match match material.get("mat") {
                        None => return Err(Malformed::Object(id, InnerError::MissingField("material/mat"))),
                        Some(material) => material,
                    }.as_array() {
                        None => return Err(Malformed::Object(id, InnerError::FieldArray("material/mat", "Material", 2))),
                        Some(material) => material,
                    };

                    let mut mat: [Material; 2] = [
                        Material::default(), Material::default()
                    ];

                    for (mat_id, material) in materials.iter().enumerate() {
                        let material = match material.as_object() {
                            None => return Err(Malformed::Object(id, InnerError::WrongField("material/mat/<element>", "Object"))),
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
                            None => 255,
                            Some(alpha) => match alpha.as_u64() {
                                None => return Err(Malformed::Object(id, InnerError::WrongField("material/mat/alpha", "u8"))),
                                Some(alpha) => alpha as u8,
                            },
                        };
                        let reflection = match material.get("reflection") {
                            None => 0,
                            Some(reflection) => match reflection.as_u64() {
                                None => return Err(Malformed::Object(id, InnerError::WrongField("material/mat/reflection", "u8"))),
                                Some(shininess) => shininess as u8,
                            },
                        };
                        let shininess = match material.get("shininess") {
                            None => 50.0,
                            Some(shininess) => match shininess.as_f64() {
                                None => return Err(Malformed::Object(id, InnerError::WrongField("material/mat/shininess", "Float"))),
                                Some(shininess) => shininess as f32,
                            },
                        };

                        mat[mat_id] = Material::new(ambient, diffuse, specular, alpha, reflection, shininess);
                    }
                    
                    let (rep_x, rep_y) = match material.get("rep") {
                        None => (
                            match material.get("repX") {
                                None => 1,
                                Some(rep) => match rep.as_u64() {
                                    None => return Err(Malformed::Object(id, InnerError::WrongField("material/repX", "Unsigned"))),
                                    Some(rep) => rep as usize,
                                }
                            },
                            match material.get("repY") {
                                None => 1,
                                Some(rep) => match rep.as_u64() {
                                    None => return Err(Malformed::Object(id, InnerError::WrongField("material/repX", "Unsigned"))),
                                    Some(rep) => rep as usize,
                                }
                            }
                        ),
                        Some(rep) => match rep.as_array() {
                            None => return Err(Malformed::Object(id, InnerError::FieldArray("material/rep", "Int", 2))),
                            Some(rep) => if rep.len() == 2 {
                                let x = match rep[0].as_u64() {
                                    None => return Err(Malformed::Object(id, InnerError::FieldArray("material/rep", "Int", 2))),
                                    Some(x) => x as usize,
                                };
                                let y = match rep[1].as_u64() {
                                    None => return Err(Malformed::Object(id, InnerError::FieldArray("material/rep", "Int", 2))),
                                    Some(x) => x as usize,
                                };

                                (x, y)
                            } else {
                                return Err(Malformed::Object(id, InnerError::FieldArray("material/rep", "Int", 2)))
                            }
                        }
                    };

                    Box::new(GridMat::new(mat[0], mat[1], rep_x, rep_y))
                }
                "TEXTURE" => {
                    let file_name = match match material.get("resource") {
                        None => return Err(Malformed::Object(id, InnerError::MissingField("material/resource"))),
                        Some(file_name) => file_name,
                    }.as_str() {
                        None => return Err(Malformed::Object(id, InnerError::WrongField("material/resource", "String"))),
                        Some(file_name) => file_name,
                    };

                    let reflection = match material.get("reflection") {
                        None => 0,
                        Some(reflection) => match reflection.as_u64() {
                            None => return Err(Malformed::Object(id, InnerError::WrongField("material/reflection", "u8"))),
                            Some(shininess) => shininess as u8,
                        },
                    };
                    let shininess = match material.get("shininess") {
                        None => 50.0,
                        Some(shininess) => match shininess.as_f64() {
                            None => return Err(Malformed::Object(id, InnerError::WrongField("material/shininess", "Float"))),
                            Some(shininess) => shininess as f32,
                        },
                    };

                    let (rep_x, rep_y) = match material.get("rep") {
                        None => (
                            match material.get("repX") {
                                None => 1,
                                Some(rep) => match rep.as_u64() {
                                    None => return Err(Malformed::Object(id, InnerError::WrongField("material/repX", "Unsigned"))),
                                    Some(rep) => rep as usize,
                                }
                            },
                            match material.get("repY") {
                                None => 1,
                                Some(rep) => match rep.as_u64() {
                                    None => return Err(Malformed::Object(id, InnerError::WrongField("material/repX", "Unsigned"))),
                                    Some(rep) => rep as usize,
                                }
                            }
                        ),
                        Some(rep) => match rep.as_array() {
                            None => return Err(Malformed::Object(id, InnerError::FieldArray("material/rep", "Int", 2))),
                            Some(rep) => if rep.len() == 2 {
                                let x = match rep[0].as_u64() {
                                    None => return Err(Malformed::Object(id, InnerError::FieldArray("material/rep", "Int", 2))),
                                    Some(x) => x as usize,
                                };
                                let y = match rep[1].as_u64() {
                                    None => return Err(Malformed::Object(id, InnerError::FieldArray("material/rep", "Int", 2))),
                                    Some(x) => x as usize,
                                };

                                (x, y)
                            } else {
                                return Err(Malformed::Object(id, InnerError::FieldArray("material/rep", "Int", 2)))
                            }
                        }
                    };

                    Box::new(Texture::new(file_name, rep_x, rep_y, reflection, shininess))
                }
                unknown => return Err(Malformed::Object(id, InnerError::UnknownMaterial(unknown.to_owned()))),
            };

            let coef_refraction = match object.get("refraction") {
                None => 1.0,
                Some(refraction) => match refraction.as_f64() {
                    None => return Err(Malformed::Object(id, InnerError::WrongField("refraction", "Float"))),
                    Some(refraction) => refraction as f32,
                }
            };

            let mut scn_object: Box<dyn Object> = match obj_type {
                "SPHERE" => Box::new(Sphere::new(mat_object, coef_refraction)),
                "PLANE" => Box::new(Plane::new(mat_object, coef_refraction)),
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

            if let Some(rotate) = object.get("rotate") {
                let rotate = match rotate.as_array() {
                    None => return Err(Malformed::Object(id, InnerError::FieldArray("rotate", "Float", 3))),
                    Some(rotate) => rotate,
                };

                if rotate.len() == 3 {
                    let x = match rotate[0].as_f64() {
                        None => return Err(Malformed::Object(id, InnerError::FieldArray("rotate", "Float", 3))),
                        Some(x) => x as f32,
                    };
                    let y = match rotate[1].as_f64() {
                        None => return Err(Malformed::Object(id, InnerError::FieldArray("rotate", "Float", 3))),
                        Some(y) => y as f32,
                    };
                    let z = match rotate[2].as_f64() {
                        None => return Err(Malformed::Object(id, InnerError::FieldArray("rotate", "Float", 3))),
                        Some(z) => z as f32,
                    };

                    scn_object.rotate_x(x);
                    scn_object.rotate_y(y);
                    scn_object.rotate_z(z);
                } else {
                    return Err(Malformed::Object(id, InnerError::FieldArray("rotate", "Float", 3)));
                }
            }

            if let Some(scale) = object.get("scale") {
                let scale = match scale.as_f64() {
                    None => return Err(Malformed::Object(id, InnerError::WrongField("scale", "Float"))),
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
                None => return Err(Malformed::Light(id, InnerError::WrongField("type", "String"))),
            };

            let color = match match light.get("color") {
                None => return Err(Malformed::Light(id, InnerError::MissingField("color"))),
                Some(color) => color,
            }.as_object() {
                None => return Err(Malformed::Light(id, InnerError::WrongField("color", "Object"))),
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

fn parse_scene(json: &Map<String, Value>) -> Result<Scene, Malformed> {
    if let Some(scene) = json.get("scene") {
        let scene = match scene.as_object() {
            None => return Err(Malformed::NotAnObject("scene")),
            Some(scene) => scene,
        };

        let background = match get_color(scene, "sky", "sky") {
            Err(inner) => match inner {
                InnerError::MissingField(_) => Color::SKY,
                _ => return Err(Malformed::Scene(inner)),
            }
            Ok(background) => background,
        };

        let ambient = match get_color(scene, "ambient", "ambient") {
            Err(inner) => match inner {
                InnerError::MissingField(_) => Color::new_gray(127),
                _ => return Err(Malformed::Scene(inner)),
            }
            Ok(ambient) => ambient,
        };

        Ok(Scene::new_custom(background, ambient))
    } else {
        Ok(Scene::new())
    }
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
                None => return Err(Malformed::Camera(InnerError::WrongField("focal", "Object"))),
                Some(focal) => {
                    let focal_type = match match focal.get("type") {
                        None => return Err(Malformed::Camera(InnerError::MissingField("focal/type"))),
                        Some(focal) => focal,
                    }.as_str() {
                        None => return Err(Malformed::Camera(InnerError::WrongField("focal/type", "String"))),
                        Some(focal) => focal,
                    };

                    let size = match match focal.get("size") {
                        None => return Err(Malformed::Camera(InnerError::MissingField("focal/size"))),
                        Some(size) => size,
                    }.as_f64() {
                        None => return Err(Malformed::Camera(InnerError::WrongField("focal/size", "Float"))),
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
                            "NO_SHADOW" => flags |= Camera::NO_SHADOW,
                            unknown => return Err(Malformed::Camera(InnerError::UnknownFlag(unknown.to_owned()))),
                        }
                    }

                    flags
                }
            }
        };

        let mut scn_camera = Camera::new(x, y, focal);
        scn_camera.set_flags(flags);

        if let Some(transform) = camera.get("transform") {
            let transform = match transform.as_array() {
                None => return Err(Malformed::Camera(InnerError::FieldArray("transform", "Float", 3))),
                Some(transform) => transform,
            };

            if transform.len() == 3 {
                let x = match transform[0].as_f64() {
                    None => return Err(Malformed::Camera(InnerError::FieldArray("transform", "Float", 3))),
                    Some(x) => x as f32,
                };
                let y = match transform[1].as_f64() {
                    None => return Err(Malformed::Camera(InnerError::FieldArray("transform", "Float", 3))),
                    Some(y) => y as f32,
                };
                let z = match transform[2].as_f64() {
                    None => return Err(Malformed::Camera(InnerError::FieldArray("transform", "Float", 3))),
                    Some(z) => z as f32,
                };

                scn_camera.move_global(x, y, z);
            } else {
                return Err(Malformed::Camera(InnerError::FieldArray("transform", "Float", 3)));
            }
        }

        if let Some(rotate) = camera.get("rotate") {
            let rotate = match rotate.as_array() {
                None => return Err(Malformed::Camera(InnerError::FieldArray("rotate", "Float", 3))),
                Some(rotate) => rotate,
            };

            if rotate.len() == 3 {
                let x = match rotate[0].as_f64() {
                    None => return Err(Malformed::Camera(InnerError::FieldArray("rotate", "Float", 3))),
                    Some(x) => x as f32,
                };
                let y = match rotate[1].as_f64() {
                    None => return Err(Malformed::Camera(InnerError::FieldArray("rotate", "Float", 3))),
                    Some(y) => y as f32,
                };
                let z = match rotate[2].as_f64() {
                    None => return Err(Malformed::Camera(InnerError::FieldArray("rotate", "Float", 3))),
                    Some(z) => z as f32,
                };

                scn_camera.rotate_x(x);
                scn_camera.rotate_y(y);
                scn_camera.rotate_z(z);
            } else {
                return Err(Malformed::Camera(InnerError::FieldArray("rotate", "Float", 3)));
            }
        }

        Ok(scn_camera)
    } else {
        Ok(Camera::new(1920, 1080, Focal::Perspective(1.7)))
    }
}

struct Config {
    output: String,
    threads: usize,
    depth: usize,
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
                None => return Err(Malformed::Config(InnerError::WrongField("output", "String"))),
                Some(output) => output.to_owned(),
            }
        };
        let threads = match conf.get("threads") {
            None => 1,
            Some(threads) => match threads.as_u64() {
                None => return Err(Malformed::Config(InnerError::WrongField("threads", "Unsigned"))),
                Some(threads) => threads as usize,
            }
        };
        let depth = match conf.get("depth") {
            None => 0,
            Some(depth) => match depth.as_u64() {
                None => return Err(Malformed::Config(InnerError::WrongField("depth", "Unsigned"))),
                Some(depth) => depth as usize,
            }
        };

        Ok(Config { output, threads, depth })
    } else {
        Ok(Config { output: "output.png".to_owned(), threads: 1, depth: 0 })
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
